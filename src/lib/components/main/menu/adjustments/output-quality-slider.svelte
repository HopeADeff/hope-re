<script lang="ts">
  import { ImageIcon } from "@lucide/svelte";
  import { Badge } from "$lib/components/ui/badge";
  import { Slider } from "$lib/components/ui/slider";
  import { cn } from "$lib/utils";

  import type { OutputQualitySliderProps } from "../types";

  let { value = $bindable([92]) }: OutputQualitySliderProps = $props();

  const qualityColour = $derived(
    value[0] < 90
      ? "bg-yellow-500"
      : value[0] < 95
      ? "bg-green-500"
      : "bg-emerald-500",
  );

  const qualityLevel = $derived(
    value[0] < 90
      ? "Standard"
      : value[0] < 95
      ? "High"
      : "Best",
  );
</script>

<div class="space-y-3">
  <div class="flex items-center justify-between">
    <div class="flex items-center gap-2">
      <div class="p-1.5 rounded-lg bg-green-500/10">
        <ImageIcon class="size-4 text-green-500" />
      </div>
      <span class="text-sm font-semibold">Output Quality</span>
    </div>
    <div class="flex items-center gap-2">
      <Badge variant="secondary" class="text-xs">
        {qualityLevel}
      </Badge>
      <Badge class={cn("font-mono text-sm text-white", qualityColour)}>
        {value[0]}
      </Badge>
    </div>
  </div>

  <div class="space-y-2">
    <Slider type="multiple"
            bind:value
            min={85}
            max={98}
            step={1} />

    <div class="flex justify-between text-xs text-muted-foreground">
      <span>85 <span class="text-yellow-500">&lpar;small&rpar;</span></span>
      <span class="font-medium text-green-500">92</span>
      <span>98 <span class="text-emerald-500">&lpar;best&rpar;</span></span>
    </div>
  </div>

  <p class="text-xs text-muted-foreground">
    Image compression quality. Higher values &equals; larger file size but better visual quality.
  </p>
</div>
