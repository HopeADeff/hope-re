<script lang="ts">
  import type { Snippet } from "svelte";

  import { DownloadIcon, LoaderCircleIcon, TriangleAlertIcon } from "@lucide/svelte";
  import { Badge } from "$lib/components/ui/badge";
  import { Button } from "$lib/components/ui/button";
  import * as Dialog from "$lib/components/ui/dialog";
  import { Progress } from "$lib/components/ui/progress";
  import { useModelDownload } from "$lib/stores/use-model-download.svelte";
  import { cn } from "$lib/utils";
  import { onMount } from "svelte";

  type Props = {
    children: Snippet;
  };

  const { children }: Props = $props();

  const models = useModelDownload();
  let mounted = $state<boolean>(false);

  const dialogOpen = $derived(models.isLoading || !models.allReady);

  onMount(() => {
    mounted = true;
  });

  $effect(() => {
    if (mounted && !models.isLoading && !models.allReady && !models.isDownloading && !models.error) {
      models.startDownload();
    }
  });

  function formatBytes(bytes: number): string {
    if (bytes === 0)
      return "0 B";
    const k = 1024;
    const sizes = ["B", "KB", "MB", "GB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return `${(bytes / k ** i).toFixed(1)} ${sizes[i]}`;
  }
</script>

{@render children()}

<Dialog.Root
  open={dialogOpen}
  onOpenChange={() => {}}
>
  <Dialog.Content
    class="max-w-md"
    showCloseButton={false}
    onInteractOutside={e => e.preventDefault()}
    onEscapeKeydown={e => e.preventDefault()}
  >
    <Dialog.Header>
      <div class="flex flex-col items-center gap-3">
        <div class="size-12 rounded-lg bg-primary/10 flex items-center justify-center">
          {#if models.error}
            <TriangleAlertIcon class="size-6 text-destructive" />
          {:else}
            <DownloadIcon class="size-6 text-primary" />
          {/if}
        </div>

        <Dialog.DialogTitle class="text-lg font-medium tracking-tight">
          {#if models.isLoading}
            Checking resources
          {:else if models.error}
            Download interrupted
          {:else if models.isDownloading}
            Preparing workspace
          {:else}
            Setting up
          {/if}
        </Dialog.DialogTitle>

        <Dialog.DialogDescription class="text-center leading-relaxed max-w-xs">
          {#if models.error}
            {models.error}
          {:else if models.isDownloading && models.currentModelName}
            Downloading {models.currentModelName} model
          {:else if models.isLoading}
            Verifying model files
          {:else}
            Initializing download
          {/if}
        </Dialog.DialogDescription>
      </div>
    </Dialog.Header>

    {#if models.isDownloading || models.error}
      <div class="space-y-4">
        <div class="space-y-2">
          <div class="flex items-center justify-between">
            <span class="text-xs text-muted-foreground">Overall progress</span>
            <Badge variant="secondary" class="font-jetbrains-mono text-xs">
              {Math.round(models.overallPercent)}&percnt;
            </Badge>
          </div>
          <Progress value={models.overallPercent} max={100} />
        </div>

        <div class="space-y-2">
          {#each models.modelProgress as model}
            {@const displayName = model.name.replace("_algorithm.onnx", "")}
            <div class="flex items-center gap-3 p-2 rounded-lg bg-muted/30">
              <div class={cn(
                "size-1.5 rounded-full shrink-0",
                model.percent >= 100 ? "bg-emerald-500" : model.percent > 0 ? "bg-primary animate-pulse" : "bg-muted-foreground/30",
              )}></div>

              <span class="text-xs text-foreground capitalize flex-1">
                {displayName}
              </span>

              <span class="text-xs text-muted-foreground font-jetbrains-mono tabular-nums">
                {#if model.percent >= 100}
                  Done
                {:else if model.totalBytes > 0}
                  {formatBytes(model.downloadedBytes)} / {formatBytes(model.totalBytes)}
                {:else if model.percent > 0}
                  {Math.round(model.percent)}&percnt;
                {:else}
                  Waiting
                {/if}
              </span>
            </div>
          {/each}
        </div>
      </div>
    {:else if models.isLoading}
      <div class="flex justify-center py-4">
        <LoaderCircleIcon class="size-5 text-muted-foreground animate-spin" />
      </div>
    {/if}

    {#if models.error}
      <Dialog.Footer>
        <Button
          variant="outline"
          size="sm"
          class="gap-2"
          onclick={() => models.startDownload()}
        >
          <DownloadIcon class="size-4" />
          Retry download
        </Button>
      </Dialog.Footer>
    {/if}
  </Dialog.Content>
</Dialog.Root>
