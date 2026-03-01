<script lang="ts">
  import { CircleArrowUpIcon, DownloadIcon } from "@lucide/svelte";
  import { Progress } from "$lib/components/ui/progress";
  import { useModelDownload } from "$lib/stores/use-model-download.svelte";
  import { useUpdater } from "$lib/stores/use-updater.svelte";
  import { cn } from "$lib/utils";

  const models = useModelDownload();
  const updater = useUpdater();
</script>

{#if models.minimized && models.needsDialog}
  <button
    onclick={() => models.restore()}
    class={cn(
      "group flex items-center gap-2 px-3 py-2 rounded-lg border bg-card shadow-lg ring-1 ring-border/50",
      "hover:bg-accent transition-all duration-200 cursor-pointer",
      "border-border",
    )}
  >
    <div class="relative flex items-center justify-center size-7 rounded-md bg-primary/15 shrink-0">
      <DownloadIcon class="size-3.5 text-primary" />
      {#if models.isDownloading}
        <span class="absolute -top-0.5 -right-0.5 size-2 rounded-full bg-primary animate-pulse"></span>
      {/if}
    </div>
    <div class="flex flex-col items-start gap-0.5 min-w-0">
      <span class="text-xs font-medium text-foreground leading-none">Models</span>
      <div class="w-20">
        <Progress value={models.overallPercent}
                  max={100}
                  class="h-1" />
      </div>
    </div>
    <span class="text-xs text-muted-foreground font-jetbrains-mono tabular-nums">
      {Math.round(models.overallPercent)}%
    </span>
  </button>
{/if}

{#if updater.minimized && updater.isActive}
  <button
    onclick={() => updater.restore()}
    class={cn(
      "group flex items-center gap-2 px-3 py-2 rounded-lg border bg-card shadow-lg ring-1 ring-border/50",
      "hover:bg-accent transition-all duration-200 cursor-pointer",
      "border-border",
    )}
  >
    <div class="relative flex items-center justify-center size-7 rounded-md bg-primary/15 shrink-0">
      <CircleArrowUpIcon class="size-3.5 text-primary" />
      <span class="absolute -top-0.5 -right-0.5 size-2 rounded-full bg-primary animate-pulse"></span>
    </div>
    <div class="flex flex-col items-start gap-0.5 min-w-0">
      <span class="text-xs font-medium text-foreground leading-none">
        {updater.isInstalling ? "Installing" : "Updating"}
      </span>
      {#if updater.isDownloading}
        <div class="w-20">
          <Progress value={updater.downloadProgress}
                    max={100}
                    class="h-1" />
        </div>
      {/if}
    </div>
    {#if updater.isDownloading}
      <span class="text-xs text-muted-foreground font-jetbrains-mono tabular-nums">
        {updater.downloadProgress}%
      </span>
    {/if}
  </button>
{/if}
