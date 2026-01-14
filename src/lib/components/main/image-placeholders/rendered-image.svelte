<script lang="ts">
  import { DownloadIcon, Maximize2Icon } from "@lucide/svelte";
  import { Button } from "$lib/components/ui/button";

  import type { RenderedImageActionsProps } from "./types";

  const { onDownload, onFullscreen }: RenderedImageActionsProps = $props();
</script>

<div class="absolute inset-0 flex items-center justify-center gap-3 z-20 pointer-events-none">
  <Button variant="secondary"
          size="icon"
          class="size-14 pointer-events-auto opacity-70 hover:opacity-100 transition-all duration-200 shadow-xl hover:shadow-2xl hover:scale-110 bg-linear-to-br from-blue-500 to-blue-600 hover:from-blue-600 hover:to-blue-700 text-white border-0"
          onclick={onDownload}>
    <DownloadIcon class="size-6" />
  </Button>

  <Button variant="secondary"
          size="icon"
          class="size-14 pointer-events-auto opacity-70 hover:opacity-100 transition-all duration-200 shadow-xl hover:shadow-2xl hover:scale-110 bg-linear-to-br from-emerald-500 to-emerald-600hover:from-emerald-600 hover:to-emerald-700 text-white border-0"
          onclick={onFullscreen}>
    <Maximize2Icon class="size-6" />
  </Button>
</div>
