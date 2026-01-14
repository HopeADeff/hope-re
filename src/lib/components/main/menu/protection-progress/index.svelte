<script lang="ts">
  import {
    CircleAlertIcon,
    CircleCheckBigIcon,
    LoaderCircleIcon,
  } from "@lucide/svelte";
  import { Badge } from "$lib/components/ui/badge";
  import { Progress } from "$lib/components/ui/progress";
  import { cn } from "$lib/utils";

  import type { ProtectionProgressProps } from "../types";

  const {
    isProcessing = false,
    progress = 0,
    status = "idle",
    message = "",
  }: ProtectionProgressProps = $props();

  const statusColour = $derived(
    status === "processing"
      ? "text-blue-500"
      : status === "success"
      ? "text-green-500"
      : status === "error"
      ? "text-red-500"
      : "text-muted-foreground",
  );

  const progressColour = $derived(
    progress < 30
      ? "[&>div]:bg-blue-500"
      : progress < 70
      ? "[&>div]:bg-orange-500"
      : "[&>div]:bg-green-500",
  );
</script>

{#if isProcessing || status !== "idle"}
  <div class="space-y-3 p-4 rounded-lg border bg-linear-to-br from-muted/30 to-muted/10 animate-in fade-in slide-in-from-top-2 duration-300">
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-2">
        {#if status === "processing"}
          <LoaderCircleIcon class="size-4 text-blue-500 animate-spin" />
          <span class="text-sm font-semibold text-blue-500">Protecting...</span>
        {:else if status === "success"}
          <CircleCheckBigIcon class="size-4 text-green-500" />
          <span class="text-sm font-semibold text-green-500">Complete!</span>
        {:else if status === "error"}
          <CircleAlertIcon class="size-4 text-red-500" />
          <span class="text-sm font-semibold text-red-500">Error</span>
        {/if}
      </div>

      <Badge variant="secondary" class="font-mono text-xs">
        {progress}&percnt;
      </Badge>
    </div>

    <Progress value={progress} class={cn("h-2", progressColour)} />

    {#if message}
      <p class={cn("text-xs", statusColour)}>
        {message}
      </p>
    {/if}

    {#if status === "processing" && progress > 0}
      {@const timeRemaining = Math.ceil((100 - progress) * 0.4)}
      <p class="text-xs text-muted-foreground">
        Estimated time remaining: &#126;{timeRemaining} minutes
      </p>
    {/if}
  </div>
{/if}
