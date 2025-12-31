<script lang="ts">
  import SystemInfoDialog from "./system-info-dialog.svelte";
  import ThemeToggle from "./theme-toggle.svelte";
</script>

<header class="h-12 px-6 border-b border-border/20 backdrop-blur-sm bg-background/95 sticky top-[30px] z-50" style="margin-top: 0;">
  <div class="h-full flex items-center justify-end gap-3">
    <SystemInfoDialog />
    <ThemeToggle />
  </div>
</header>
