<script lang="ts">
  import { CircleArrowUpIcon, LoaderCircleIcon, MinimizeIcon } from "@lucide/svelte";
  import { Button } from "$lib/components/ui/button";
  import * as Dialog from "$lib/components/ui/dialog";
  import { Progress } from "$lib/components/ui/progress";
  import { useUpdater } from "$lib/stores/use-updater.svelte";
  import { parseMarkdown } from "$lib/utils";

  const updater = useUpdater();
  let hasAutoOpened = false;

  const renderedNotes = $derived.by(() => {
    if (!updater.releaseNotes)
      return "";
    return parseMarkdown(updater.releaseNotes);
  });

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
  <Dialog.Content
    class="max-w-sm max-h-[85vh] flex flex-col overflow-hidden"
    showCloseButton={!updater.isActive}
  >
    {#if updater.isActive}
      <button
        onclick={() => updater.minimize()}
        class="absolute inset-s-4 top-4 rounded-lg opacity-50 transition-opacity hover:opacity-100 focus:outline-hidden cursor-pointer"
        aria-label="Minimize to dock"
      >
        <MinimizeIcon class="size-4" />
      </button>
    {/if}

    <Dialog.Header class="shrink-0">
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

    <div class="mt-4 space-y-4 min-h-0 flex-1 overflow-y-auto">
      {#if renderedNotes}
        <div class="rounded-lg border bg-card p-4">
          <p class="text-xs font-medium text-muted-foreground mb-2">What's new</p>
          <div class="release-notes text-sm leading-relaxed">
            {@html renderedNotes}
          </div>
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

    <Dialog.Footer class="mt-4 shrink-0">
      {#if updater.isActive}
        <Button
          variant="outline"
          size="sm"
          onclick={() => updater.minimize()}
          class="gap-2"
        >
          <MinimizeIcon class="size-3.5" />
          Minimize
        </Button>
      {:else if !updater.isDownloading && !updater.isInstalling}
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

<style>
  .release-notes :global(h1),
  .release-notes :global(h2),
  .release-notes :global(h3) {
    font-weight: 600;
    margin-top: 0.75rem;
    margin-bottom: 0.25rem;
  }
  .release-notes :global(h1) {
    font-size: 1.125rem;
  }
  .release-notes :global(h2) {
    font-size: 1rem;
  }
  .release-notes :global(h3) {
    font-size: 0.875rem;
  }
  .release-notes :global(h1:first-child),
  .release-notes :global(h2:first-child),
  .release-notes :global(h3:first-child) {
    margin-top: 0;
  }
  .release-notes :global(p) {
    margin-bottom: 0.5rem;
  }
  .release-notes :global(p:last-child) {
    margin-bottom: 0;
  }
  .release-notes :global(ul),
  .release-notes :global(ol) {
    padding-left: 1.25rem;
    margin-bottom: 0.5rem;
  }
  .release-notes :global(ul) {
    list-style-type: disc;
  }
  .release-notes :global(ol) {
    list-style-type: decimal;
  }
  .release-notes :global(li) {
    margin-bottom: 0.25rem;
  }
  .release-notes :global(code) {
    font-size: 0.8em;
    padding: 0.125rem 0.375rem;
    border-radius: 0.25rem;
    background-color: var(--muted);
    font-family: var(--font-jetbrains-mono);
  }
  .release-notes :global(pre) {
    padding: 0.75rem;
    border-radius: 0.25rem;
    background-color: var(--muted);
    overflow-x: auto;
    margin-bottom: 0.5rem;
  }
  .release-notes :global(pre code) {
    padding: 0;
    background-color: transparent;
  }
  .release-notes :global(a) {
    color: var(--primary);
    text-decoration: underline;
    text-underline-offset: 2px;
  }
  .release-notes :global(a:hover) {
    opacity: 0.8;
  }
  .release-notes :global(blockquote) {
    border-left: 2px solid var(--border);
    padding-left: 0.75rem;
    margin-bottom: 0.5rem;
    color: var(--muted-foreground);
  }
  .release-notes :global(hr) {
    border-color: var(--border);
    margin: 0.75rem 0;
  }
  .release-notes :global(strong) {
    font-weight: 600;
  }
</style>
