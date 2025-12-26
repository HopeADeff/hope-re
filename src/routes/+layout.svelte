<script lang="ts">
  import type { Snippet } from "svelte";

  import { platform } from "@tauri-apps/plugin-os";
  import { WindowTitle } from "$lib/components";

  import "../app.css";

  const { children }: { children: Snippet } = $props();

  let currentPlatform = $state<string>("");
  const isWindows = $derived(currentPlatform === "windows");

  if (typeof window !== "undefined") {
    currentPlatform = platform();
  }
</script>

{#if isWindows}
  <WindowTitle />
{/if}
{@render children()}
