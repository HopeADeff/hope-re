<script lang="ts">
  import { CircleArrowUpIcon, LoaderCircleIcon } from "@lucide/svelte";
  import { Button } from "$lib/components/ui/button";
  import * as Dialog from "$lib/components/ui/dialog";
  import { Progress } from "$lib/components/ui/progress";
  import { useUpdater } from "$lib/stores/use-updater.svelte";

  const updater = useUpdater();
  let hasAutoOpened = false;

  $effect(() => {
    if (updater.isUpdateAvailable && !hasAutoOpened) {
      hasAutoOpened = true;
      updater.openDialog();
    }
  });
</script>

<Dialog.Root
  open={updater.dialogOpen}
  onOpenChange={(open) => {
    if (!open)
      updater.dismiss();
  }}
>
  <Dialog.Content class="max-w-sm">
    <Dialog.Header>
      <div class="flex items-center gap-3">
        <div class="p-2 rounded-lg bg-primary/10">
          <CircleArrowUpIcon class="size-4" />
        </div>
        <div>
          <Dialog.DialogTitle class="text-lg font-bold">
            New Version
          </Dialog.DialogTitle>
          {#if updater.version}
            <p class="text-xs text-muted-foreground mt-0.5">v{updater.version}</p>
          {/if}
        </div>
      </div>
    </Dialog.Header>

    <div class="mt-4 space-y-4">
      {#if updater.releaseNotes}
        <div class="rounded-lg border bg-card p-4">
          <p class="text-xs font-medium text-muted-foreground mb-2">What's new</p>
          <p class="text-sm whitespace-pre-line leading-relaxed">{updater.releaseNotes}</p>
        </div>
      {/if}

      {#if updater.isDownloading}
        <div class="space-y-2">
          <div class="flex items-center justify-between text-xs text-muted-foreground">
            <span>Downloading</span>
            <span class="font-jetbrains-mono">{updater.downloadProgress}%</span>
          </div>
          <Progress value={updater.downloadProgress} />
        </div>
      {/if}

      {#if updater.isInstalling}
        <div class="flex items-center gap-2 text-sm text-muted-foreground">
          <LoaderCircleIcon class="size-4 animate-spin" />
          <span>Installing and restarting</span>
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
          <CircleArrowUpIcon class="size-3.5" />
          Update
        </Button>
      {/if}
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>
