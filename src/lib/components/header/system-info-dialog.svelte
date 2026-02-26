<script lang="ts">
  import {
    CircleAlertIcon,
    CpuIcon,
    GlobeIcon,
    HardDriveIcon,
    InfoIcon,
    LoaderCircleIcon,
    MemoryStickIcon,
    MonitorIcon,
    RefreshCwIcon,
    ZapIcon,
  } from "@lucide/svelte";
  import { Button } from "$lib/components/ui/button";
  import * as Dialog from "$lib/components/ui/dialog";
  import { useSystemInfo } from "$lib/queries/system-info";
  import { cn } from "$lib/utils";

  const {
    data: info,
    isLoading,
    isError,
    error,
    isFetching,
    refetch,
  } = $derived(useSystemInfo());

  let dialogOpen = $state<boolean>(false);

  function handleDialogOpen(open: boolean) {
    dialogOpen = open;
  }
</script>

<Dialog.Root open={dialogOpen} onOpenChange={handleDialogOpen}>
  <Dialog.Trigger>
    <Button variant="outline"
            size="sm"
            class="flex items-center gap-2 px-3 py-2 rounded-lg border border-border hover:bg-muted/50 hover:cursor-pointer transition-colors"
            aria-label="System Info">
      <InfoIcon class="size-4" />
    </Button>
  </Dialog.Trigger>

  <Dialog.Content class="max-w-md">
    <Dialog.Header>
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-3">
          <div class="p-2 rounded-lg bg-primary/10">
            <MonitorIcon class="size-4" />
          </div>
          <Dialog.DialogTitle class="text-lg font-bold">
            System Information
          </Dialog.DialogTitle>
        </div>
      </div>
    </Dialog.Header>

    <div class="mt-4">
      {#if isLoading && !info}
        <div class="flex flex-col items-center justify-center py-12">
          <LoaderCircleIcon class="size-10 animate-spin text-primary mb-3" />
          <p class="text-sm text-muted-foreground">Loading system info...</p>
        </div>
      {:else if isError}
        <div class="flex items-start gap-3 p-4 rounded-lg bg-destructive/10 border border-destructive/20">
          <CircleAlertIcon class="size-5 text-destructive shrink-0 mt-0.5" />
          <div>
            <p class="text-sm font-medium text-destructive">Failed to load system information</p>
            <p class="text-xs text-muted-foreground mt-1">{error?.message || "Unknown Error"}</p>
          </div>
        </div>
      {:else if info}
        <div class={cn(isFetching && "opacity-50 transition-opacity")}>
          <div class="rounded-lg border bg-card p-6 space-y-4">
            <div class="pb-4 border-b">
              <div class="flex items-center gap-2 mb-2">
                <GlobeIcon class="size-4 text-sky-600 dark:text-sky-400" />
                <p class="text-xs font-medium text-muted-foreground">Platform</p>
              </div>
              <p class="text-sm font-semibold pl-6">{info.platform.os}</p>
              <p class="text-xs text-muted-foreground mt-0.5 pl-6">{info.platform.hostname}</p>
            </div>

            <div class="space-y-3.5">
              <div class="flex items-start gap-3">
                <CpuIcon class="size-4 text-indigo-600 dark:text-indigo-400 shrink-0 mt-0.5" />
                <div class="flex-1 min-w-0">
                  <p class="text-sm">
                    <span class="font-medium text-muted-foreground">CPU: </span>
                    <span class="font-semibold ml-2">{info.cpu}</span>
                  </p>
                </div>
              </div>

              <div class="flex items-start gap-3">
                <MemoryStickIcon class="size-4 text-violet-600 dark:text-violet-400 shrink-0 mt-0.5" />
                <div class="flex-1 min-w-0">
                  <p class="text-sm">
                    <span class="font-medium text-muted-foreground">Memory:</span>
                    <span class="font-semibold ml-2">{info.memory}</span>
                  </p>
                </div>
              </div>

              <div class="flex items-start gap-3">
                <ZapIcon class="size-4 text-emerald-600 dark:text-emerald-400 shrink-0 mt-0.5" />
                <div class="flex-1 min-w-0">
                  <p class="text-sm">
                    <span class="font-medium text-muted-foreground">GPU: </span>
                    <span class="font-semibold ml-2">{info.gpu}</span>
                  </p>
                </div>
              </div>

              <div class="flex items-start gap-3">
                <HardDriveIcon class="size-4 text-amber-600 dark:text-amber-400 shrink-0 mt-0.5" />
                <div class="flex-1 min-w-0">
                  <p class="text-sm">
                    <span class="font-medium text-muted-foreground">Storage:</span>
                    <span class="font-semibold ml-2">{info.storage}</span>
                  </p>
                </div>
              </div>
            </div>
          </div>
        </div>
      {/if}
    </div>

    {#if info && !isLoading}
      <div class="flex items-center justify-between pt-4 border-t">
        <p class="text-xs text-muted-foreground">
          Last updated: {new Date().toLocaleTimeString()}
        </p>
        <Button
          variant="outline"
          size="sm"
          onclick={refetch}
          disabled={isFetching}
          class="gap-2"
        >
          <RefreshCwIcon class={cn("size-3.5", isFetching && "animate-spin")} />
          <span>Refresh</span>
        </Button>
      </div>
    {/if}
  </Dialog.Content>
</Dialog.Root>
