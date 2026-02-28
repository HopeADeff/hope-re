<script lang="ts">
  import { DownloadIcon, Maximize2Icon } from "@lucide/svelte";
  import { Button } from "$lib/components/ui/button";

  import type { RenderedImageActionsProps } from "./types";

  const { onDownload, onFullscreen }: RenderedImageActionsProps = $props();
</script>

<div class="absolute inset-0 flex items-center justify-center gap-4 z-20 pointer-events-none opacity-0 hover:opacity-100 transition-opacity duration-500">
  <Button variant="ghost"
          size="icon"
          class="size-10 pointer-events-auto rounded-lg bg-muted/60 hover:bg-muted/80 border border-border/20 backdrop-blur-sm shadow-sm"
          onclick={onDownload}>
    <DownloadIcon class="size-4" />
  </Button>

  <Button variant="ghost"
          size="icon"
          class="size-10 pointer-events-auto rounded-lg bg-muted/60 hover:bg-muted/80 border border-border/20 backdrop-blur-sm shadow-sm"
          onclick={onFullscreen}>
    <Maximize2Icon class="size-4" />
  </Button>
</div>
