import type { Update } from "@tauri-apps/plugin-updater";

import { relaunch } from "@tauri-apps/plugin-process";
import { check } from "@tauri-apps/plugin-updater";
import { toast } from "svelte-sonner";

type UpdateStatus = "idle" | "checking" | "available" | "downloading" | "installing" | "error";

export function useUpdater() {
  let status = $state<UpdateStatus>("idle");
  let update = $state<Update | null>(null);
  let downloadProgress = $state<number>(0);
  let contentLength = $state<number>(0);
  let error = $state<string | null>(null);

  const isUpdateAvailable = $derived(status === "available");
  const isDownloading = $derived(status === "downloading");
  const isInstalling = $derived(status === "installing");
  const isChecking = $derived(status === "checking");
  const version = $derived(update?.version ?? null);
  const releaseNotes = $derived(update?.body ?? null);

  async function checkForUpdate() {
    if (status === "checking" || status === "downloading" || status === "installing")
      return;

    status = "checking";
    error = null;

    try {
      const result = await check();
      if (result) {
        update = result;
        status = "available";
      }
      else {
        status = "idle";
      }
    }
    catch (e) {
      error = e instanceof Error ? e.message : String(e);
      status = "error";
      console.error("Update check failed:", e);
    }
  }

  async function downloadAndInstall() {
    if (!update || status === "downloading" || status === "installing")
      return;

    status = "downloading";
    downloadProgress = 0;
    contentLength = 0;
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

      status = "installing";
      toast.success("Update installed, restarting...");
      await relaunch();
    }
    catch (e) {
      error = e instanceof Error ? e.message : String(e);
      status = "error";
      toast.error("Update failed");
      console.error("Update download/install failed:", e);
    }
  }

  function dismiss() {
    if (status === "available") {
      status = "idle";
      update = null;
    }
  }

  return {
    get status() {
      return status;
    },
    get isUpdateAvailable() {
      return isUpdateAvailable;
    },
    get isDownloading() {
      return isDownloading;
    },
    get isInstalling() {
      return isInstalling;
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
    checkForUpdate,
    downloadAndInstall,
    dismiss,
  };
}
