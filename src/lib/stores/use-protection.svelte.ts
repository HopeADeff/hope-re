import type {
  ProtectionMenuProps,
  ProtectionProgressProps,
} from "$lib/components";

import { buildProtectionSettings, useProtectImage } from "$lib/queries";
import { toast } from "svelte-sonner";

export type ProtectionState = {
  algorithm: Exclude<ProtectionMenuProps["algorithm"], undefined>;
  glazeStyle: Exclude<ProtectionMenuProps["glazeStyle"], undefined>;
  nightshadeTarget: Exclude<ProtectionMenuProps["nightshadeTarget"], undefined>;
  intensity: number[];
  outputQuality: number[];
  renderQuality: number[];
};

const DEFAULTS: ProtectionState = {
  algorithm: "noise",
  glazeStyle: "abstract",
  nightshadeTarget: "dog",
  intensity: [20],
  outputQuality: [92],
  renderQuality: [50],
};

const PROGRESS_TICK_MS = 500;
const SUCCESS_RESET_MS = 3000;
const ERROR_RESET_MS = 5000;

export function useProtection() {
  let algorithm = $state<ProtectionState["algorithm"]>(DEFAULTS.algorithm);
  let glazeStyle = $state<ProtectionState["glazeStyle"]>(DEFAULTS.glazeStyle);
  let nightshadeTarget = $state<ProtectionState["nightshadeTarget"]>(DEFAULTS.nightshadeTarget);
  let intensity = $state<number[]>(DEFAULTS.intensity);
  let outputQuality = $state<number[]>(DEFAULTS.outputQuality);
  let renderQuality = $state<number[]>(DEFAULTS.renderQuality);

  let progress = $state<number>(0);
  let progressStatus = $state<ProtectionProgressProps["status"]>("idle");
  let progressMessage = $state<string>("");

  let progressInterval: ReturnType<typeof setInterval> | null = null;

  const mutation = useProtectImage();

  const isProcessing = $derived(mutation.isPending);
  const resultImage = $derived(mutation.data?.image_base64 ?? null);
  const hasResult = $derived(mutation.isSuccess && !!mutation.data?.image_base64);

  function startProgressSimulation() {
    progressInterval = setInterval(() => {
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
    }, PROGRESS_TICK_MS);
  }

  function stopProgressSimulation() {
    if (progressInterval) {
      clearInterval(progressInterval);
      progressInterval = null;
    }
  }

  async function handleProtect(imageBase64: string) {
    progress = 0;
    progressStatus = "processing";
    progressMessage = "Initializing protection...";
    toast.info("Starting image protection...");

    startProgressSimulation();

    try {
      const settings = buildProtectionSettings({
        algorithm,
        glazeStyle,
        nightshadeTarget,
        intensity,
        outputQuality,
        renderQuality,
      });

      const result = await mutation.mutateAsync({
        imageBase64,
        settings,
      });

      stopProgressSimulation();

      if (!result.success) {
        throw new Error(result.message);
      }

      progress = 100;
      progressMessage = "Protection complete!";
      progressStatus = "success";

      toast.success("Image protected successfully!");

      setTimeout(() => {
        if (progressStatus === "success") {
          progressStatus = "idle";
          progress = 0;
          progressMessage = "";
        }
      }, SUCCESS_RESET_MS);
    }
    catch (error) {
      stopProgressSimulation();
      progress = 0;
      progressStatus = "error";
      progressMessage = "Failed to protect image. Please try again.";
      toast.error("Protection failed");
      console.error("Protection error:", error);

      setTimeout(() => {
        if (progressStatus === "error") {
          progressStatus = "idle";
          progressMessage = "";
        }
      }, ERROR_RESET_MS);
    }
  }

  function resetSettings() {
    algorithm = DEFAULTS.algorithm;
    glazeStyle = DEFAULTS.glazeStyle;
    nightshadeTarget = DEFAULTS.nightshadeTarget;
    intensity = [...DEFAULTS.intensity];
    outputQuality = [...DEFAULTS.outputQuality];
    renderQuality = [...DEFAULTS.renderQuality];
  }

  function resetProgress() {
    stopProgressSimulation();
    progress = 0;
    progressStatus = "idle";
    progressMessage = "";
    mutation.reset();
  }

  return {
    get algorithm() {
      return algorithm;
    },
    set algorithm(value: ProtectionState["algorithm"]) {
      algorithm = value;
    },
    get glazeStyle() {
      return glazeStyle;
    },
    set glazeStyle(value: ProtectionState["glazeStyle"]) {
      glazeStyle = value;
    },
    get nightshadeTarget() {
      return nightshadeTarget;
    },
    set nightshadeTarget(value: ProtectionState["nightshadeTarget"]) {
      nightshadeTarget = value;
    },
    get intensity() {
      return intensity;
    },
    set intensity(value: number[]) {
      intensity = value;
    },
    get outputQuality() {
      return outputQuality;
    },
    set outputQuality(value: number[]) {
      outputQuality = value;
    },
    get renderQuality() {
      return renderQuality;
    },
    set renderQuality(value: number[]) {
      renderQuality = value;
    },
    get progress() {
      return progress;
    },
    get progressStatus() {
      return progressStatus;
    },
    get progressMessage() {
      return progressMessage;
    },
    get isProcessing() {
      return isProcessing;
    },
    get resultImage() {
      return resultImage;
    },
    get hasResult() {
      return hasResult;
    },
    handleProtect,
    resetSettings,
    resetProgress,
  };
}
