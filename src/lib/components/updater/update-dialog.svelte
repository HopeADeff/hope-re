<script lang="ts">
  import { DownloadIcon, LoaderCircleIcon, RefreshCwIcon } from "@lucide/svelte";
  import { Button } from "$lib/components/ui/button";
  import * as Dialog from "$lib/components/ui/dialog";
  import { Progress } from "$lib/components/ui/progress";
  import { useUpdater } from "$lib/stores/use-updater.svelte";
  import { onMount } from "svelte";

  const updater = useUpdater();

  onMount(() => {
    updater.checkForUpdate();
  });
</script>

<Dialog.Root
  open={updater.isUpdateAvailable}
  onOpenChange={(open) => {
    if (!open)
      updater.dismiss();
  }}
>
  <Dialog.Content class="max-w-sm">
    <Dialog.Header>
      <div class="flex items-center gap-3">
        <div class="p-2 rounded-lg bg-primary/10">
          <DownloadIcon class="size-4" />
        </div>
        <Dialog.DialogTitle class="text-lg font-bold">
          Update Available
        </Dialog.DialogTitle>
      </div>
      <Dialog.Description class="text-sm text-muted-foreground mt-2">
        Version {updater.version} is ready to install.
      </Dialog.Description>
    </Dialog.Header>

    <div class="mt-4 space-y-4">
      {#if updater.releaseNotes}
        <div class="rounded-lg border bg-card p-4">
          <p class="text-xs font-medium text-muted-foreground mb-2">Release Notes</p>
          <p class="text-sm whitespace-pre-line">{updater.releaseNotes}</p>
        </div>
      {/if}

      {#if updater.isDownloading}
        <div class="space-y-2">
          <div class="flex items-center justify-between text-xs text-muted-foreground">
            <span>Downloading...</span>
            <span>{updater.downloadProgress}%</span>
          </div>
          <Progress value={updater.downloadProgress} />
        </div>
      {/if}

      {#if updater.isInstalling}
        <div class="flex items-center gap-2 text-sm text-muted-foreground">
          <LoaderCircleIcon class="size-4 animate-spin" />
          <span>Installing and restarting...</span>
        </div>
      {/if}

      {#if updater.error}
        <div class="p-3 rounded-lg bg-destructive/10 border border-destructive/20">
          <p class="text-sm text-destructive">{updater.error}</p>
        </div>
      {/if}
    </div>

    <Dialog.Footer class="mt-4">
      {#if !updater.isDownloading && !updater.isInstalling}
        <Button
          variant="outline"
          size="sm"
          onclick={() => updater.dismiss()}
        >
          Later
        </Button>
        <Button
          size="sm"
          onclick={() => updater.downloadAndInstall()}
          class="gap-2"
        >
          <RefreshCwIcon class="size-3.5" />
          Update Now
        </Button>
      {/if}
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>
