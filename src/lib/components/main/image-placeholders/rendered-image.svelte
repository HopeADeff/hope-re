<script lang="ts">
  import { DownloadIcon, Maximize2Icon } from "@lucide/svelte";
  import { Button } from "$lib/components/ui/button";

  import type { RenderedImageActionsProps } from "./types";

  const { onDownload, onFullscreen }: RenderedImageActionsProps = $props();
</script>

<div class="absolute inset-0 flex items-center justify-center gap-3 z-20 pointer-events-none">
  <Button variant="secondary"
          size="icon"
          class="size-14 pointer-events-auto opacity-70 hover:opacity-100 transition-opacity bg-indigo-600/80 hover:bg-indigo-600/90 dark:bg-indigo-500/80 dark:hover:bg-indigo-500/90 text-white border-0"
          onclick={onDownload}>
    <DownloadIcon class="size-6" />
  </Button>

  <Button variant="secondary"
          size="icon"
          class="size-14 pointer-events-auto opacity-70 hover:opacity-100 transition-opacity bg-emerald-600/70 hover:bg-emerald-600/80 dark:bg-emerald-500/70 dark:hover:bg-emerald-500/80 text-white border-0"
          onclick={onFullscreen}>
    <Maximize2Icon class="size-6" />
  </Button>
</div>
