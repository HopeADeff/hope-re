import { useQueryClient } from "@tanstack/svelte-query";
import { relaunch } from "@tauri-apps/plugin-process";
import { useCheckForUpdate } from "$lib/queries";
import { toast } from "svelte-sonner";

let dialogOpen = $state<boolean>(false);
let downloadProgress = $state<number>(0);
let contentLength = $state<number>(0);
let downloadStatus = $state<"idle" | "downloading" | "installing" | "error">("idle");
let downloadError = $state<string | null>(null);
let minimized = $state<boolean>(false);

export function useUpdater() {
  const checkQuery = useCheckForUpdate();
  const queryClient = useQueryClient();

  const update = $derived(checkQuery.data ?? null);
  const isUpdateAvailable = $derived(!!update);
  const isChecking = $derived(checkQuery.isFetching);
  const isDownloading = $derived(downloadStatus === "downloading");
  const isInstalling = $derived(downloadStatus === "installing");
  const isActive = $derived(isDownloading || isInstalling);
  const version = $derived(update?.version ?? null);
  const releaseNotes = $derived(update?.body ?? null);
  const error = $derived(downloadError ?? (checkQuery.isError ? String(checkQuery.error) : null));

  async function checkForUpdate(manual = false) {
    if (downloadStatus === "downloading" || downloadStatus === "installing")
      return;

    try {
      const result = await queryClient.fetchQuery({
        queryKey: ["updater-check"],
        staleTime: 0,
      });

      if (result) {
        dialogOpen = true;
      }
      else if (manual) {
        toast.info("You are on the latest version");
      }
    }
    catch (e) {
      if (manual) {
        toast.error("Failed to check for updates");
      }
      console.error("Update check failed:", e);
    }
  }

  async function downloadAndInstall() {
    if (!update || downloadStatus === "downloading" || downloadStatus === "installing")
      return;

    downloadStatus = "downloading";
    downloadProgress = 0;
    contentLength = 0;
    downloadError = null;
    let downloaded = 0;

    try {
      await update.downloadAndInstall((event) => {
        switch (event.event) {
          case "Started":
            contentLength = event.data.contentLength ?? 0;
            break;
          case "Progress":
            downloaded += event.data.chunkLength;
            downloadProgress = contentLength > 0
              ? Math.round((downloaded / contentLength) * 100)
              : 0;
            break;
          case "Finished":
            downloadProgress = 100;
            break;
        }
      });

      downloadStatus = "installing";
      toast.success("Update installed, restarting...");
      await relaunch();
    }
    catch (e) {
      downloadError = e instanceof Error ? e.message : String(e);
      downloadStatus = "error";
      toast.error("Update failed");
      console.error("Update download/install failed:", e);
    }
  }

  function dismiss() {
    if (isActive) {
      minimized = true;
      return;
    }
    dialogOpen = false;
    minimized = false;
    if (downloadStatus === "error") {
      downloadStatus = "idle";
      downloadError = null;
    }
  }

  function minimize() {
    minimized = true;
    dialogOpen = false;
  }

  function restore() {
    minimized = false;
    dialogOpen = true;
  }

  function openDialog() {
    if (isUpdateAvailable) {
      dialogOpen = true;
    }
    else {
      checkForUpdate(true);
    }
  }

  return {
    get isUpdateAvailable() {
      return isUpdateAvailable;
    },
    get isDownloading() {
      return isDownloading;
    },
    get isInstalling() {
      return isInstalling;
    },
    get isActive() {
      return isActive;
    },
    get isChecking() {
      return isChecking;
    },
    get version() {
      return version;
    },
    get releaseNotes() {
      return releaseNotes;
    },
    get downloadProgress() {
      return downloadProgress;
    },
    get error() {
      return error;
    },
    get dialogOpen() {
      return dialogOpen;
    },
    get minimized() {
      return minimized;
    },
    checkForUpdate,
    downloadAndInstall,
    dismiss,
    minimize,
    restore,
    openDialog,
  };
}
