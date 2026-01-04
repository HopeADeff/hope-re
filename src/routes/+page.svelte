<script lang="ts">
  import { LoaderCircleIcon, ShieldIcon, XIcon } from "@lucide/svelte";
  import {
    BaseImagePlaceholder,
    ImageFullscreenDialog,
    ProtectionMenu,
    RenderedImageActions,
  } from "$lib/components";
  import { Button } from "$lib/components/ui/button";
  import { toast } from "svelte-sonner";

  let originalImage = $state<string | null>(null);
  let renderedImage = $state<string | null>(null);
  let isProcessing = $state<boolean>(false);
  let fullscreenOpen = $state<boolean>(false);

  let progress = $state<number>(0);
  let progressStatus = $state<"idle" | "processing" | "success" | "error">("idle");
  let progressMessage = $state<string>("");

  let algorithm = $state("glaze");
  let targetStyle = $state("abstract");
  let targetDescription = $state("");
  let inputPrompt = $state("");
  let intensity = $state([20]);
  let outputQuality = $state([92]);
  let renderQuality = $state([50]);

  let isMobile = $state<boolean>(false);

  $effect(() => {
    const mediaQuery = window.matchMedia("(max-width: 1023px)");
    isMobile = mediaQuery.matches;

    function handler(e: MediaQueryListEvent) {
      isMobile = e.matches;
    }

    mediaQuery.addEventListener("change", handler);
    return () => mediaQuery.removeEventListener("change", handler);
  });

  function handleFileSelect(files: FileList) {
    const file = files[0];

    if (!file || !file.type.startsWith("image/")) {
      toast.error("Please select a valid image file");
      return;
    }

    const maxSize = 10 * 1024 * 1024;
    if (file.size > maxSize) {
      toast.error("File size must be less than 10MB");
      return;
    }

    const reader = new FileReader();

    reader.onload = (e) => {
      originalImage = e.target?.result as string;
      renderedImage = null;
      progress = 0;
      progressStatus = "idle";
      progressMessage = "";
      toast.success(`Loaded ${file.name}`);
    };

    reader.onerror = () => {
      toast.error("Failed to load image");
    };

    reader.readAsDataURL(file);
  }

  function getProtectionSettings() {
    const baseSettings = {
      algorithm,
      intensity: intensity[0] / 100,
      outputQuality: outputQuality[0],
      renderQuality: renderQuality[0],
    };

    switch (algorithm) {
      case "glaze":
        return { ...baseSettings, targetStyle };
      case "noise":
        return { ...baseSettings, targetDescription };
      case "nightshade":
        return { ...baseSettings, inputPrompt };
      default:
        return baseSettings;
    }
  }

  async function handleProtect() {
    if (!originalImage)
      return;

    // Validate algorithm-specific inputs
    if (algorithm === "noise" && !targetDescription.trim()) {
      toast.error("Please enter a target description");
      return;
    }

    if (algorithm === "nightshade" && !inputPrompt.trim()) {
      toast.error("Please enter an input prompt");
      return;
    }

    isProcessing = true;
    progress = 0;
    progressStatus = "processing";
    progressMessage = "Initializing protection...";
    toast.info("Starting image protection...");

    try {
      const settings = getProtectionSettings();
      // eslint-disable-next-line no-console
      console.log("Protection settings:", settings);

      const progressInterval = setInterval(() => {
        if (progress < 90) {
          progress += Math.random() * 10;
          progress = Math.min(progress, 90);

          if (progress < 20) {
            progressMessage = "Analyzing image structure...";
          }
          else if (progress < 40) {
            progressMessage = "Preparing protection algorithm...";
          }
          else if (progress < 60) {
            progressMessage = `Applying ${algorithm} protection...`;
          }
          else if (progress < 80) {
            progressMessage = "Processing pixel data...";
          }
          else {
            progressMessage = "Finalizing output...";
          }
        }
      }, 500);

      // TODO: Call Rust backend

      const processingTime = 2000 + (renderQuality[0] * 50);
      await new Promise(resolve => setTimeout(resolve, processingTime));

      clearInterval(progressInterval);
      progress = 100;
      progressMessage = "Protection complete!";
      progressStatus = "success";

      renderedImage = originalImage;

      toast.success("Image protected successfully!");

      setTimeout(() => {
        if (progressStatus === "success") {
          progressStatus = "idle";
          progress = 0;
          progressMessage = "";
        }
      }, 3000);
    }
    catch (error) {
      progress = 0;
      progressStatus = "error";
      progressMessage = "Failed to protect image.  Please try again.";
      toast.error("Protection failed");
      console.error("Protection error:", error);

      setTimeout(() => {
        if (progressStatus === "error") {
          progressStatus = "idle";
          progressMessage = "";
        }
      }, 5000);
    }
    finally {
      isProcessing = false;
    }
  }

  function handleCancel() {
    if (isProcessing) {
      toast.info("Protection cancelled");
    }

    isProcessing = false;
    originalImage = null;
    renderedImage = null;
    progress = 0;
    progressStatus = "idle";
    progressMessage = "";

    algorithm = "glaze";
    targetStyle = "abstract";
    targetDescription = "";
    inputPrompt = "";
    intensity = [20];
    outputQuality = [92];
    renderQuality = [50];

    toast.success("All cleared");
  }

  function handleDownload() {
    if (!renderedImage)
      return;

    try {
      const link = document.createElement("a");
      link.href = renderedImage;

      const timestamp = new Date().toISOString().replace(/[:.]/g, "-").slice(0, -5);
      link.download = `protected-${algorithm}-${timestamp}.png`;

      link.click();

      toast.success("Image downloaded successfully");
    }
    catch (error) {
      toast.error("Failed to download image");
      console.error("Download error:", error);
    }
  }

  function handleFullscreen() {
    fullscreenOpen = true;
  }

  const hasImage = $derived(!!originalImage);
  const canProcess = $derived(hasImage && !isProcessing);
</script>

<svelte:head>
  <title>Hope:RE</title>
</svelte:head>

<main class="flex-1 overflow-auto">
  <div class="container mx-auto p-4 md:p-6 h-full max-w-7xl">
    <div class="flex flex-col gap-6 h-full">
      {#if isMobile}
        <div class="flex flex-col gap-4 flex-1 min-h-0">
          {#if renderedImage}
            <BaseImagePlaceholder imageSrc={renderedImage}
                                  label="Protected Image"
                                  readonly>
              <RenderedImageActions onDownload={handleDownload}
                                    onFullscreen={handleFullscreen} />
            </BaseImagePlaceholder>
          {:else}
            <BaseImagePlaceholder imageSrc={originalImage}
                                  label="Original Image"
                                  onFileSelect={handleFileSelect} />
          {/if}
        </div>
      {:else}
        <div class="grid grid-cols-2 gap-6 flex-1">
          <BaseImagePlaceholder imageSrc={originalImage}
                                label="Original Image"
                                onFileSelect={handleFileSelect} />

          <BaseImagePlaceholder imageSrc={renderedImage}
                                label="Protected Image"
                                readonly>
            {#if renderedImage}
              <RenderedImageActions onDownload={handleDownload}
                                    onFullscreen={handleFullscreen} />
            {/if}
          </BaseImagePlaceholder>
        </div>
      {/if}

      <ProtectionMenu bind:algorithm
                      bind:targetStyle
                      bind:targetDescription
                      bind:inputPrompt
                      bind:intensity
                      bind:outputQuality
                      bind:renderQuality
                      {isProcessing}
                      {progress}
                      status={progressStatus}
                      progressMessage={progressMessage} />

      <div class="grid grid-cols-2 gap-4 pb-4">
        <Button size="lg"
                class="gap-2 h-14 bg-linear-to-r from-primary to-primary/80 hover:from-primary/90 hover:to-primary/70 shadow-lg hover:shadow-xl transition-all duration-200 hover:scale-[1.02] disabled:opacity-50 disabled:cursor-not-allowed disabled:hover:scale-100"
                onclick={handleProtect}
                disabled={!canProcess}>
          {#if isProcessing}
            <LoaderCircleIcon class="size-5 animate-spin" />
            <span class="font-semibold">Processing...</span>
          {:else}
            <ShieldIcon class="size-5" />
            <span class="font-semibold">Protect Image</span>
          {/if}
        </Button>

        <Button variant="outline"
                size="lg"
                class="gap-2 h-14 border-2 hover:bg-destructive/10 hover:border-destructive/50 hover:text-destructive transition-all duration-200 hover:scale-[1.02] disabled:opacity-50 disabled:cursor-not-allowed disabled:hover:scale-100"
                onclick={handleCancel}
                disabled={!hasImage}>
          <XIcon class="size-5" />
          <span class="font-semibold">Cancel</span>
        </Button>
      </div>
    </div>
  </div>
</main>

<ImageFullscreenDialog bind:open={fullscreenOpen}
                       imageSrc={renderedImage} />
