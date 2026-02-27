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
  let panX = $state<number>(0);
  let panY = $state<number>(0);
  let isDragging = $state<boolean>(false);
  let startX = $state<number>(0);
  let startY = $state<number>(0);

  function handleZoomIn() {
    zoom = Math.min(zoom + 25, 300);
  }

  function handleZoomOut() {
    zoom = Math.max(zoom - 25, 50);
  }

  function handleReset() {
    zoom = 100;
    panX = 0;
    panY = 0;
  }

  function handleMouseDown(e: MouseEvent) {
    if (zoom > 100) {
      isDragging = true;
      startX = e.clientX - panX;
      startY = e.clientY - panY;
    }
  }

  function handleMouseMove(e: MouseEvent) {
    if (isDragging) {
      panX = e.clientX - startX;
      panY = e.clientY - startY;
    }
  }

  function handleMouseUp() {
    isDragging = false;
  }

  function handleTouchStart(e: TouchEvent) {
    if (zoom > 100 && e.touches.length === 1) {
      isDragging = true;
      startX = e.touches[0].clientX - panX;
      startY = e.touches[0].clientY - panY;
    }
  }

  function handleTouchMove(e: TouchEvent) {
    if (isDragging && e.touches.length === 1) {
      panX = e.touches[0].clientX - startX;
      panY = e.touches[0].clientY - startY;
    }
  }

  function handleTouchEnd() {
    isDragging = false;
  }
</script>

<Dialog.Root bind:open {onOpenChange}>
  <Dialog.Content class="max-w-[95vw] max-h-[95vh] w-full h-full p-0" showCloseButton={false}>
    <div class="relative w-full h-full flex flex-col bg-background">
      <div class="absolute top-4 right-4 z-50">
        <Button variant="ghost"
                size="icon"
                class="size-10 bg-muted hover:bg-muted/80"
                onclick={() => open = false}>
          <XIcon class="size-5" />
        </Button>
      </div>

      <div class="absolute top-4 left-1/2 -translate-x-1/2 z-50">
        <div class="flex items-center gap-2 bg-muted backdrop-blur rounded-lg p-2">
          <Button variant="ghost"
                  size="icon"
                  class="size-8"
                  onclick={handleZoomOut}
                  disabled={zoom <= 50}>
            <ZoomOutIcon class="size-4" />
          </Button>

          <Button variant="ghost"
                  size="sm"
                  class="h-8 min-w-[60px]"
                  onclick={handleReset}>
            {zoom}%
          </Button>

          <Button variant="ghost"
                  size="icon"
                  class="size-8"
                  onclick={handleZoomIn}
                  disabled={zoom >= 300}>
            <ZoomInIcon class="size-4" />
          </Button>
        </div>
      </div>

      <div class="flex flex-1 items-center justify-center overflow-hidden p-8"
           onmousedown={handleMouseDown}
           onmousemove={handleMouseMove}
           onmouseup={handleMouseUp}
           onmouseleave={handleMouseUp}
           ontouchstart={handleTouchStart}
           ontouchmove={handleTouchMove}
           ontouchend={handleTouchEnd}
           role="presentation">
        {#if imageSrc}
          <img src={imageSrc}
               alt="Fullscreen Preview"
               class="max-w-full max-h-full object-contain transition-transform duration-200 select-none {isDragging ? "cursor-grabbing" : "cursor-grab"}"
               style="transform: translate({panX}px, {panY}px) scale({zoom / 100})"
               draggable="false" />
        {/if}
      </div>
    </div>
  </Dialog.Content>
</Dialog.Root>
