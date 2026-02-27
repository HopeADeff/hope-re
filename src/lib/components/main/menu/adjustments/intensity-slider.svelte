<script lang="ts">
  import { GaugeIcon } from "@lucide/svelte";
  import { Badge } from "$lib/components/ui/badge";
  import { Slider } from "$lib/components/ui/slider";
  import { cn } from "$lib/utils";

  import type { IntensitySliderProps } from "../types";

  let { value = $bindable([20]) }: IntensitySliderProps = $props();

  const sliderId = `intensity-slider-${Math.random().toString(36).substring(2, 9)}`;

  const intensityDisplay = $derived((value[0] / 100).toFixed(2));

  const intensityLevel = $derived(
    value[0] < 9
      ? "Low"
      : value[0] < 17
      ? "Medium"
      : "High",
  );
</script>

<div class="space-y-3">
  <div class="flex items-center justify-between">
    <div class="flex items-center gap-2">
      <div class="p-1.5 rounded-lg bg-amber-500/10">
        <GaugeIcon class="size-4 text-amber-600 dark:text-amber-400" />
      </div>
      <label for={sliderId} class="text-sm font-medium">Intensity</label>
    </div>
    <div class="flex items-center gap-2">
      <Badge variant="secondary"
             class={cn(
               "text-xs",
               intensityLevel === "Low" && "text-emerald-600 dark:text-emerald-400",
               intensityLevel === "Medium" && "text-amber-600 dark:text-amber-400",
               intensityLevel === "High" && "text-rose-600 dark:text-rose-400",
             )}>
        {intensityLevel}
      </Badge>
      <Badge variant="secondary" class="font-jetbrains-mono text-sm">
        {intensityDisplay}
      </Badge>
    </div>
  </div>

  <div class="space-y-2">
    <Slider id={sliderId}
            type="multiple"
            bind:value
            min={1}
            max={25}
            step={1}
            aria-label="Intensity control" />

    <div class="flex justify-between items-center text-xs text-muted-foreground">
      <span>0.01</span>
      <span class="font-medium">0.12</span>
      <span>0.25</span>
    </div>
  </div>

  <p class="text-xs text-muted-foreground">
    Controls the strength of protection. Higher values &equals; stronger protection but more visible changes.
  </p>
</div>
