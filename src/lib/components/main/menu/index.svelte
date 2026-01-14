<script lang="ts">
  import { Settings2Icon, SparklesIcon } from "@lucide/svelte";
  import { Badge } from "$lib/components/ui/badge";
  import { Separator } from "$lib/components/ui/separator";

  import type { ProtectionMenuProps } from "./types";

  import {
    AlgorithmSelect,
    InputPrompt,
    IntensitySlider,
    OutputQualitySlider,
    RenderQualitySlider,
    TargetDescriptionInput,
    TargetStyleSelect,
  } from "./adjustments";
  import { ProtectionProgress } from "./protection-progress";

  let {
    algorithm = $bindable("noise"),
    targetStyle = $bindable("abstract"),
    targetDescription = $bindable(""),
    inputPrompt = $bindable(""),
    intensity = $bindable([20]),
    outputQuality = $bindable([92]),
    renderQuality = $bindable([50]),
    isProcessing = false,
    progress = 0,
    status = "idle",
    progressMessage = "",
  }: ProtectionMenuProps = $props();

  const showTargetStyle = $derived(algorithm === "glaze");
  const showTargetDescription = $derived(algorithm === "noise");
  const showInputPrompt = $derived(algorithm === "nightshade");
</script>

<div class="space-y-6">
  <div class="space-y-6 p-6 border rounded-xl bg-linear-to-br from-card via-card to-card/80 shadow-lg">
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-3">
        <div class="p-2 rounded-lg bg-linear-to-br from-primary/10 to-primary/5">
          <Settings2Icon class="size-5 text-primary" />
        </div>
        <h3 class="text-base font-bold">Protection Settings</h3>
      </div>
      <Badge variant="outline" class="gap-1.5 shadow-sm">
        <SparklesIcon class="size-3" />
        <span class="text-xs font-semibold">Advanced</span>
      </Badge>
    </div>

    <Separator class="bg-linear-to-r from-transparent via-border to-transparent" />

    <AlgorithmSelect bind:value={algorithm} />

    <Separator class="bg-linear-to-r from-transparent via-border to-transparent" />

    {#if showTargetStyle}
      <div class="animate-in fade-in slide-in-from-top-2 duration-300">
        <TargetStyleSelect bind:value={targetStyle} />
      </div>
    {:else if showTargetDescription}
      <div class="animate-in fade-in slide-in-from-top-2 duration-300">
        <TargetDescriptionInput bind:value={targetDescription} />
      </div>
    {:else if showInputPrompt}
      <div class="animate-in fade-in slide-in-from-top-2 duration-300">
        <InputPrompt bind:value={inputPrompt} />
      </div>
    {/if}

    <Separator class="bg-linear-to-r from-transparent via-border to-transparent" />

    <IntensitySlider bind:value={intensity} />

    <Separator class="bg-linear-to-r from-transparent via-border to-transparent" />

    <OutputQualitySlider bind:value={outputQuality} />

    <RenderQualitySlider bind:value={renderQuality} />
  </div>

  <ProtectionProgress {isProcessing}
                      {progress}
                      {status}
                      message={progressMessage} />
</div>
