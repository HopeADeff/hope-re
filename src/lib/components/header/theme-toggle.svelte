<script lang="ts">
  import type { Theme } from "$lib/stores/theme.svelte";

  import { MoonIcon, SunIcon } from "@lucide/svelte";
  import { Button } from "$lib/components/ui/button";
  import { useTheme } from "$lib/stores/theme.svelte";

  const theme = useTheme();

  const currentTheme = $derived<Theme>(theme.getTheme());

  const isDark = $derived(
    currentTheme === "dark" || (currentTheme === "system" && typeof window !== "undefined" && window.matchMedia("(prefers-color-scheme: dark)").matches),
  );

  async function toogleTheme() {
    const newTheme: Theme = isDark ? "light" : "dark";
    await theme.setTheme(newTheme);
  }
</script>

<Button variant="outline"
        size="sm"
        onclick={toogleTheme}
        class="flex items-center gap-2 px-3 py-2 rounded-lg border border-border hover:bg-muted/50 hover:cursor-pointer transition-colors"
        aria-label="Toggle theme">
  {#if isDark}
    <SunIcon class="size-4 transition-all duration-300 rotate-0 hover:rotate-90" />
  {:else}
    <MoonIcon class="size-4 transition-all duration-300 rotate-0 hover:-rotate-12" />
  {/if}
</Button>

<!-- svelte-ignore css_unused_selector -->
<style>
  :global(.dark) button :global(svg) {
    filter: drop-shadow(0 0 8px rgba(139, 92, 246, 0.5));
  }
</style>
