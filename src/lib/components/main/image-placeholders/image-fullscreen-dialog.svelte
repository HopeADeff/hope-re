<script lang="ts">
  import {
    XIcon,
    ZoomInIcon,
    ZoomOutIcon,
  } from "@lucide/svelte";
  import { Button } from "$lib/components/ui/button";
  import * as Dialog from "$lib/components/ui/dialog";

  import type { ImageFullscreenDialogProps } from "./types";

  let {
    open = $bindable(false),
    imageSrc,
    onOpenChange,
  }: ImageFullscreenDialogProps = $props();

  let zoom = $state<number>(100);

  function handleZoomIn() {
    zoom = Math.min(zoom + 25, 300);
  }

  function handleZoomOut() {
    zoom = Math.max(zoom - 25, 50);
  }

  function handleReset() {
    zoom = 100;
  }
</script>

<Dialog.Root bind:open {onOpenChange}>
  <Dialog.Content class="max-w-[95vw] max-h-[95vh] w-full h-full p-0">
    <div class="relative w-full h-full flex flex-col bg-black/95">
      <div class="absolute top-4 right-4 z-50">
        <Button variant="ghost"
                size="icon"
                class="size-10 bg-white/10 hover:bg-white/20 text-white"
                onclick={() => open = false}>
          <XIcon class="size-5" />
        </Button>
      </div>

      <div class="absolute top-4 left-1/2 -translate-x-1/2 z-50">
        <div class="flex items-center gap-2 bg-white/10 backdrop-blur rounded-lg p-2">
          <Button variant="ghost"
                  size="icon"
                  class="size-8 text-white hover:bg-white/20"
                  onclick={handleZoomOut}
                  disabled={zoom <= 50}>
            <ZoomOutIcon class="size-4" />
          </Button>

          <Button variant="ghost"
                  size="sm"
                  class="h-8 min-w-[60px] text-white hover:bg-white/20"
                  onclick={handleReset}>
            {zoom}&percnt;
          </Button>

          <Button variant="ghost"
                  size="icon"
                  class="size-8 text-white hover:bg-white/20"
                  onclick={handleZoomIn}
                  disabled={zoom >= 300}>
            <ZoomInIcon class="size-4" />
          </Button>
        </div>
      </div>

      <div class="flex flex-1 items-center justify-center overflow-auto p-8">
        {#if imageSrc}
          <img src={imageSrc}
               alt="Fullscreen Preview"
               class="max-w-none transition-transform duration-200"
               style="transform: scale({zoom / 100})" />
        {/if}
      </div>
    </div>
  </Dialog.Content>
</Dialog.Root>
