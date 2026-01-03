<script lang="ts">
  import { PaletteIcon } from "@lucide/svelte";
  import * as Select from "$lib/components/ui/select";
  import { cn } from "$lib/utils";

  type AlgorithmSelectProps = {
    value?: string;
  };

  let { value = $bindable("noise") }: AlgorithmSelectProps = $props();

  const algorithms: {
    value: string;
    label: string;
    description: string;
    colour: string;
    bgColour: string;
  }[] = [
    {
      value: "noise",
      label: "Noise",
      description: "Add adversarial noise",
      colour: "text-blue-500",
      bgColour: "bg-blue-500/10",
    },
    {
      value: "glaze",
      label: "Glaze",
      description: "Style transfer protection",
      colour: "text-purple-500",
      bgColour: "bg-purple-500/10",
    },
    {
      value: "nightshade",
      label: "Nightshade",
      description: "Data poisoning protection",
      colour: "text-red-500",
      bgColour: "bg-red-500/10",
    },
  ];

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
    <span class="text-sm font-semibold">Algorithm</span>
  </div>

  <Select.Root type="single" bind:value>
    <Select.Trigger class="w-full hover:border-primary/30 transition-colors" aria-label="Algorithm Selection">
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
