<script lang="ts">
  import { CircleArrowUpIcon, LoaderCircleIcon } from "@lucide/svelte";
  import { Button } from "$lib/components/ui/button";
  import { useUpdater } from "$lib/stores/use-updater.svelte";
  import { cn } from "$lib/utils";

  const updater = useUpdater();
</script>

<Button
  variant="outline"
  size="sm"
  onclick={() => updater.openDialog()}
  disabled={updater.isDownloading || updater.isInstalling}
  class={cn(
    "flex items-center gap-2 px-3 py-2 rounded-lg border border-border hover:bg-muted/50 hover:cursor-pointer transition-colors",
    updater.isUpdateAvailable && "border-primary/40 text-primary",
  )}
  aria-label="Check for updates"
>
  {#if updater.isChecking}
    <LoaderCircleIcon class="size-4 animate-spin" />
  {:else}
    <CircleArrowUpIcon class={cn("size-4", updater.isUpdateAvailable && "text-primary")} />
  {/if}
</Button>
