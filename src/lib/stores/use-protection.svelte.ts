import type {
  ProtectionMenuProps,
  ProtectionProgressProps,
} from "$lib/components";

import { listen } from "@tauri-apps/api/event";
import { buildProtectionSettings, useProtectImage } from "$lib/queries";
import { toast } from "svelte-sonner";

type ProtectionProgress = {
  stage: string;
  tile_current: number;
  tile_total: number;
  iteration_current: number;
  iteration_total: number;
  percent: number;
};

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
  intensity: [50],
  outputQuality: [92],
  renderQuality: [75],
};

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

  let progressInterval: (() => void) | null = null;

  const mutation = useProtectImage();

  const isProcessing = $derived(mutation.isPending);
  const resultImage = $derived(mutation.data?.image_base64 ?? null);
  const hasResult = $derived(mutation.isSuccess && !!mutation.data?.image_base64);
  const modelUsed = $derived(mutation.data?.model_used ?? true);
  const resultMessage = $derived(mutation.data?.message ?? "");

  function stageMessage(stage: string): string {
    switch (stage) {
      case "loading":
        return "Loading model...";
      case "processing":
        return `Applying ${algorithm} protection...`;
      case "encoding":
        return "Encoding output...";
      case "complete":
        return "Protection complete!";
      default:
        return "Processing...";
    }
  }

  async function startProgressListener() {
    const unlisten = await listen<ProtectionProgress>(
      "protection-progress",
      (event) => {
        const data = event.payload;
        progress = Math.round(data.percent);
        progressMessage = stageMessage(data.stage);
      },
    );
    progressInterval = unlisten;
  }

  function stopProgressListener() {
    if (progressInterval) {
      progressInterval();
      progressInterval = null;
    }
  }

  async function handleProtect(imageBase64: string) {
    mutation.reset();
    progress = 0;
    progressStatus = "processing";
    progressMessage = "Initializing protection...";
    toast.info("Starting image protection...");

    await startProgressListener();

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

      stopProgressListener();

      if (!result.success) {
        throw new Error(result.message);
      }

      progress = 100;
      progressMessage = "Protection complete!";
      progressStatus = "success";

      if (result.model_used) {
        toast.success("Image protected successfully!");
      }
      else {
        toast.warning("Image protected with basic fallback. Download AI models for stronger protection.");
      }

      setTimeout(() => {
        if (progressStatus === "success") {
          progressStatus = "idle";
          progress = 0;
          progressMessage = "";
        }
      }, SUCCESS_RESET_MS);
    }
    catch (error) {
      stopProgressListener();
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
    stopProgressListener();
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
    get modelUsed() {
      return modelUsed;
    },
    get resultMessage() {
      return resultMessage;
    },
    handleProtect,
    resetSettings,
    resetProgress,
  };
}
