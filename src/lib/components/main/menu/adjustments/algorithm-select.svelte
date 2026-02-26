<script lang="ts">
  import { PaletteIcon } from "@lucide/svelte";
  import * as Select from "$lib/components/ui/select";
  import { algorithms } from "$lib/constants";
  import { cn } from "$lib/utils";

  import type { AlgorithmSelectProps } from "../types";

  let { value = $bindable("noise") }: AlgorithmSelectProps = $props();

  const triggerContent = $derived(
    algorithms.find(algo => algo.value === value)?.label ?? "Select an algorithm",
  );

  const currentAlgo = $derived(
    algorithms.find(algo => algo.value === value) ?? algorithms[0],
  );
</script>

<div class="space-y-3">
  <div class="flex items-center gap-2">
    <div class={cn("p-1.5 rounded-lg", currentAlgo.bgColour)}>
      <PaletteIcon class={cn("size-4", currentAlgo.colour)} />
    </div>
    <span class="text-sm font-medium">Algorithm</span>
  </div>

  <Select.Root type="single" bind:value>
    <Select.Trigger class="w-full hover:border-foreground/20 transition-colors" aria-label="Algorithm Selection">
      {triggerContent}
    </Select.Trigger>
    <Select.Content>
      {#each algorithms as algo}
        <Select.Item value={algo.value}>
          <div class="flex items-center gap-3 py-1">
            <div class={cn("p-1.5 rounded", algo.bgColour)}>
              <PaletteIcon class={cn("size-4", algo.colour)} />
            </div>
            <div class="flex-1">
              <p class="font-medium text-sm">{algo.label}</p>
              <p class="text-xs text-muted-foreground">{algo.description}</p>
            </div>
          </div>
        </Select.Item>
      {/each}
    </Select.Content>
  </Select.Root>
</div>
