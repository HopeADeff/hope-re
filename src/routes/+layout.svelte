<script lang="ts">
  import type { Snippet } from "svelte";

  import { platform } from "@tauri-apps/plugin-os";
  import { Header, WindowTitle } from "$lib/components";

  import "../app.css";
  import { themeSetting } from "$lib/stores/theme.svelte";
  import { onMount } from "svelte";

  const { children }: { children: Snippet } = $props();

  let currentPlatform = $state<string>("");
  const isWindows = $derived(currentPlatform === "windows");

  if (typeof window !== "undefined") {
    currentPlatform = platform();
  }

  onMount(async () => {
    await themeSetting.initTheme();
  });
</script>

{#if isWindows}
  <WindowTitle />
{/if}
<Header />
{@render children()}
