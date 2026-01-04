<script lang="ts">
  import SystemInfoDialog from "./system-info-dialog.svelte";
  import ThemeToggle from "./theme-toggle.svelte";
</script>

<header class="border-b border-border/20 backdrop-blur-sm bg-background/95 shrink-0 z-50">
  <div class="h-[env(safe-area-inset-top)]"></div>
  <div class="h-12 px-6 flex items-center justify-end gap-3">
    <SystemInfoDialog />
    <ThemeToggle />
  </div>
</header>
