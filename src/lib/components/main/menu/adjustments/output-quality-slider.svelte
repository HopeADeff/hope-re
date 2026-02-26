<script lang="ts">
  import { ImageIcon } from "@lucide/svelte";
  import { Badge } from "$lib/components/ui/badge";
  import { Slider } from "$lib/components/ui/slider";
  import { cn } from "$lib/utils";

  import type { OutputQualitySliderProps } from "../types";

  let { value = $bindable([92]) }: OutputQualitySliderProps = $props();

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
      <div class="p-1.5 rounded-lg bg-emerald-500/10">
        <ImageIcon class="size-4 text-emerald-600 dark:text-emerald-400" />
      </div>
      <span class="text-sm font-medium">Output Quality</span>
    </div>
    <div class="flex items-center gap-2">
      <Badge variant="secondary"
             class={cn(
               "text-xs",
               qualityLevel === "Standard" && "text-amber-600 dark:text-amber-400",
               qualityLevel === "High" && "text-emerald-600 dark:text-emerald-400",
               qualityLevel === "Best" && "text-sky-600 dark:text-sky-400",
             )}>
        {qualityLevel}
      </Badge>
      <Badge variant="secondary" class="font-mono text-sm">
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
      <span>85</span>
      <span class="font-medium">92</span>
      <span>98</span>
    </div>
  </div>

  <p class="text-xs text-muted-foreground">
    Image compression quality. Higher values &equals; larger file size but better visual quality.
  </p>
</div>
