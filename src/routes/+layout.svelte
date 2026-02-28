<script lang="ts">
  import type { Snippet } from "svelte";

  import { QueryClientProvider } from "@tanstack/svelte-query";
  import { SvelteQueryDevtools } from "@tanstack/svelte-query-devtools";
  import { platform } from "@tauri-apps/plugin-os";
  import { Header, ResourceDownloadGuard, UpdateDialog, WindowTitle } from "$lib/components";

  import "../app.css";
  import { Toaster } from "$lib/components/ui/sonner";
  import { useTheme } from "$lib/stores/theme.svelte";
  import { cn } from "$lib/utils";
  import { onMount } from "svelte";

  import type { LayoutData } from "./$types";

  type LayoutDataProps = {
    children: Snippet;
    data: LayoutData;
  };

  const { children, data }: LayoutDataProps = $props();

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

<QueryClientProvider client={data.queryClient}>
  <Toaster position="top-center" />
  <UpdateDialog />

  {#if isWindows}
    <WindowTitle />
  {/if}

  <div class={cn("h-screen flex flex-col overflow-hidden", isWindows && "pt-[30px]")}>
    <Header />
    <main class="flex-1 overflow-auto">
      <ResourceDownloadGuard>
        {@render children()}
      </ResourceDownloadGuard>
    </main>
  </div>

  {#if import.meta.env.DEV}
    <SvelteQueryDevtools initialIsOpen={false} />
  {/if}
</QueryClientProvider>
