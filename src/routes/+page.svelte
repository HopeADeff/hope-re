<script lang="ts">
  import {
    LoaderCircleIcon,
    ShieldIcon,
    XIcon,
  } from "@lucide/svelte";
  import {
    BaseImagePlaceholder,
    ImageFullscreenDialog,
    ProtectionMenu,
    RenderedImageActions,
  } from "$lib/components";
  import { Button } from "$lib/components/ui/button";
  import { useInferenceCapabilities } from "$lib/queries";
  import { useImage } from "$lib/stores/use-image.svelte";
  import { useProtection } from "$lib/stores/use-protection.svelte";
  import { toast } from "svelte-sonner";

  const image = useImage();
  const protection = useProtection();

  let isMobile = $state<boolean>(false);

  const {
    data: inferenceData,
    isSuccess,
  } = $derived(useInferenceCapabilities());

  $effect(() => {
    const mediaQuery = window.matchMedia("(max-width: 1023px)");
    isMobile = mediaQuery.matches;

    function handler(e: MediaQueryListEvent) {
      isMobile = e.matches;
    }

    mediaQuery.addEventListener("change", handler);
    return () => mediaQuery.removeEventListener("change", handler);
  });

  function handleProtect() {
    if (!image.originalImage)
      return;

    protection.handleProtect(image.originalImage);
  }

  function handleCancel() {
    if (protection.isProcessing) {
      toast.info("Protection cancelled");
    }

    protection.resetProgress();
    protection.resetSettings();
    image.clear();

    toast.success("All cleared");
  }

  function handleDownload() {
    image.handleDownload(protection.resultImage, protection.algorithm);
  }

  const canProcess = $derived(image.hasImage && !protection.isProcessing);
</script>

<svelte:head>
  <title>Hope:RE</title>
  <meta name="description" content="Protect your images from unauthorized AI training with advanced algorithms like Glaze and Nightshade." />
</svelte:head>

<div class="w-full bg-background">
  <div class="container mx-auto p-4 md:p-6 h-full max-w-7xl">
    <div class="flex flex-col gap-6 h-full">
      {#if isMobile}
        <div class="flex flex-col gap-4 flex-1 min-h-0">
          {#if protection.resultImage}
            <BaseImagePlaceholder imageSrc={protection.resultImage}
                                  label="Protected Image"
                                  readonly>
              <RenderedImageActions onDownload={handleDownload}
                                    onFullscreen={image.handleFullscreen} />
            </BaseImagePlaceholder>
          {:else}
            <BaseImagePlaceholder imageSrc={image.originalImage}
                                  label="Original Image"
                                  onUpload={image.handleUpload} />
          {/if}
        </div>
      {:else}
        <div class="grid grid-cols-2 gap-6 flex-1">
          <BaseImagePlaceholder imageSrc={image.originalImage}
                                label="Original Image"
                                onUpload={image.handleUpload} />

          <BaseImagePlaceholder imageSrc={protection.resultImage}
                                label="Protected Image"
                                readonly>
            {#if protection.resultImage}
              <RenderedImageActions onDownload={handleDownload}
                                    onFullscreen={image.handleFullscreen} />
            {/if}
          </BaseImagePlaceholder>
        </div>
      {/if}

      <ProtectionMenu bind:algorithm={protection.algorithm}
                      bind:glazeStyle={protection.glazeStyle}
                      bind:nightshadeTarget={protection.nightshadeTarget}
                      bind:intensity={protection.intensity}
                      bind:outputQuality={protection.outputQuality}
                      bind:renderQuality={protection.renderQuality}
                      isProcessing={protection.isProcessing}
                      progress={protection.progress}
                      status={protection.progressStatus}
                      progressMessage={protection.progressMessage} />

      {#if isSuccess && inferenceData}
        <div class="text-xs text-muted-foreground px-2">
          <span class="font-medium">Inference:</span>
          {#each inferenceData.providers as provider}
            <span class="ml-2 inline-flex items-center gap-1">
              {provider.name}
              <span class="size-1.5 rounded-full bg-emerald-500"></span>
            </span>
          {/each}
        </div>
      {/if}

      <div class="grid grid-cols-2 gap-4 pb-4">
        <Button
          size="lg"
          class="gap-2 h-14 bg-primary hover:bg-primary/90 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          onclick={handleProtect}
          disabled={!canProcess}
        >
          {#if protection.isProcessing}
            <LoaderCircleIcon class="size-5 animate-spin" />
            <span class="font-medium">Processing...</span>
          {:else}
            <ShieldIcon class="size-5" />
            <span class="font-medium">Protect Image</span>
          {/if}
        </Button>

        <Button
          variant="outline"
          size="lg"
          class="gap-2 h-14 border hover:bg-destructive/10 hover:border-destructive/30 hover:text-destructive transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          onclick={handleCancel}
          disabled={!image.hasImage}
        >
          <XIcon class="size-5" />
          <span class="font-medium">Cancel</span>
        </Button>
      </div>
    </div>
  </div>
</div>

<ImageFullscreenDialog bind:open={image.fullscreenOpen}
                       imageSrc={protection.resultImage} />
