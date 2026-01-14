<script lang="ts">
  import { ClockIcon } from "@lucide/svelte";
  import { Badge } from "$lib/components/ui/badge";
  import { Slider } from "$lib/components/ui/slider";
  import { qualityPresets } from "$lib/constants";
  import { cn } from "$lib/utils";

  import type { RenderQualitySliderProps } from "../types";

  let { value = $bindable([50]) }: RenderQualitySliderProps = $props();

  const sliderId = `render-slider-${Math.random().toString(36).substring(2, 9)}`;

  const currentPreset = $derived(
    qualityPresets.find(p => p.value === value[0]) || qualityPresets[2],
  );

  const badgeVariant = $derived(
    value[0] <= 25
      ? "default"
      : value[0] === 50
      ? "secondary"
      : "outline",
  );
</script>

<div class="space-y-3">
  <div class="flex items-center justify-between">
    <div class="flex items-center gap-2">
      <div class="p-1.5 rounded-lg bg-blue-500/10">
        <ClockIcon class="size-4 text-blue-500" />
      </div>
      <label for={sliderId} class="text-sm font-semibold">Render Quality</label>
    </div>
    <Badge variant={badgeVariant} class="gap-1.5">
      {@const Icon = currentPreset.icon}
      <Icon class="size-3" />
      <span class="text-xs font-semibold">{currentPreset.label}</span>
    </Badge>
  </div>

  <p class="text-xs text-muted-foreground">
    Processing time for Glaze/Nightshade/Noise. Higher = better quality but slower.
  </p>

  <div class="space-y-3">
    <Slider type="multiple"
            bind:value
            min={0}
            max={100}
            step={25}
            aria-label="Render quality control" />

    <div class="flex justify-between items-start">
      {#each qualityPresets as preset}
        {@const Icon = preset.icon}
        <button type="button"
                class={cn(
                  "flex flex-col items-center gap-1 text-xs transition-all hover:scale-105",
                  (value[0] === preset.value) && "opacity-100",
                  (value[0] !== preset.value) && "opacity-50",
                )}
                onclick={() => value = [preset.value]}>
          <Icon class={cn("size-3.5", preset.colour)} />
          <span class={cn("font-medium whitespace-nowrap", preset.colour)}>
            {preset.label}
          </span>
          <span class="text-muted-foreground whitespace-nowrap">
            {preset.time}
          </span>
        </button>
      {/each}
    </div>
  </div>

  <div class="p-3 rounded-lg bg-linear-to-br from-muted/50 to-muted/30 border">
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-2">
        <currentPreset.icon class={cn("size-4", currentPreset.colour)} />
        <span class={cn("text-sm font-semibold", currentPreset.colour)}>
          {currentPreset.label}
        </span>
      </div>
      <span class="text-sm font-medium text-muted-foreground">
        {currentPreset.time}
      </span>
    </div>
  </div>
</div>
