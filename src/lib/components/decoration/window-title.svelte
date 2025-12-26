<script lang="ts">
  import {
    MaximizeIcon,
    MinimizeIcon,
    MinusIcon,
    XIcon,
  } from "@lucide/svelte";
  import { getCurrentWindow } from "@tauri-apps/api/window";

  const appWindow = getCurrentWindow();

  let isMaximized = $state<boolean>(false);

  function handleMaximize() {
    isMaximized = !isMaximized;
    appWindow.toggleMaximize();
  }
</script>

<div
  data-tauri-drag-region
  class="absolute inset-0 inset-x-0 left-0 right-0 top-0 flex h-[30px] select-none justify-end bg-background"
>
  <button
    onclick={() => appWindow.minimize()}
    class="inline-flex h-[30px] w-[30px] items-center justify-center px-2 py-1 transition-colors duration-100 hover:bg-zinc-600/50"
  >
    <MinusIcon class="size-4 text-foreground" />
  </button>
  <button
    onclick={handleMaximize}
    class="inline-flex h-[30px] w-[30px] items-center justify-center px-2 py-1 transition-colors duration-100 hover:bg-zinc-600/50"
  >
    {#if isMaximized === true}
      <MinimizeIcon class="size-4 text-foreground" />
    {:else}
      <MaximizeIcon class="size-4 text-foreground" />
    {/if}
  </button>
  <button
    onclick={() => appWindow.close()}
    class="inline-flex h-[30px] w-[30px] items-center justify-center px-2 py-1 transition-colors duration-100 hover:bg-red-500"
  >
    <XIcon class="size-4 text-foreground" />
  </button>
</div>
