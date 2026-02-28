<script lang="ts">
  import { platform } from "@tauri-apps/plugin-os";
  import { cn } from "$lib/utils";

  import SystemInfoDialog from "./system-info-dialog.svelte";
  import ThemeToggle from "./theme-toggle.svelte";
  import UpdateButton from "./update-button.svelte";

  let currentPlatform = $state<string>("");
  const isMobile = $derived(currentPlatform === "android" || currentPlatform === "ios");

  if (typeof window !== "undefined") {
    currentPlatform = platform();
  }
</script>

<header class="border-b border-border/20 backdrop-blur-sm bg-background/95 shrink-0 z-50">
  <div class={cn("h-[env(safe-area-inset-top)]", isMobile ? "min-h-6" : "")}></div>
  <div class="h-12 px-6 flex items-center justify-end gap-3">
    <UpdateButton />
    <SystemInfoDialog />
    <ThemeToggle />
  </div>
</header>
