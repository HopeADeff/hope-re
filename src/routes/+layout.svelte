<script lang="ts">
  import type { Snippet } from "svelte";

  import { platform } from "@tauri-apps/plugin-os";
  import { Header, WindowTitle } from "$lib/components";

  import "../app.css";
  import { Toaster } from "$lib/components/ui/sonner";
  import { useTheme } from "$lib/stores/theme.svelte";
  import { cn } from "$lib/utils";
  import { onMount } from "svelte";

  const { children }: { children: Snippet } = $props();

  let currentPlatform = $state<string>("");
  const isWindows = $derived(currentPlatform === "windows");

  if (typeof window !== "undefined") {
    currentPlatform = platform();
  }

  const theme = useTheme();

  onMount(async () => {
    await theme.initTheme();
  });
</script>

<Toaster position="top-center" />

{#if isWindows}
  <WindowTitle />
{/if}

<div class={cn("h-screen flex flex-col overflow-hidden", isWindows && "pt-[30px]")}>
  <Header />
  <main class="flex-1 overflow-auto">
    {@render children()}
  </main>
</div>
