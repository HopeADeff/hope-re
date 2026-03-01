import type { ModelsCheckResult } from "$lib/queries";

import { useQueryClient } from "@tanstack/svelte-query";
import { listen } from "@tauri-apps/api/event";
import { useDownloadModel, useModelsStatus } from "$lib/queries";

type DownloadProgress = {
  model_name: string;
  downloaded_bytes: number;
  total_bytes: number;
  progress_percent: number;
};

type ModelProgress = {
  name: string;
  percent: number;
  downloadedBytes: number;
  totalBytes: number;
};

const MODEL_NAMES = [
  "noise_algorithm.onnx",
  "glaze_algorithm.onnx",
  "nightshade_algorithm.onnx",
];

export function useModelDownload() {
  const statusQuery = useModelsStatus();
  const downloadMutation = useDownloadModel();
  const queryClient = useQueryClient();

  let isDownloading = $state<boolean>(false);
  let currentModelIndex = $state<number>(0);
  let modelProgress = $state<ModelProgress[]>(
    MODEL_NAMES.map(name => ({
      name,
      percent: 0,
      downloadedBytes: 0,
      totalBytes: 0,
    })),
  );
  let error = $state<string | null>(null);
  let minimized = $state<boolean>(false);

  const allReady = $derived(statusQuery.data?.all_ready ?? false);
  const isLoading = $derived(statusQuery.isLoading);
  const totalModels = MODEL_NAMES.length;

  const overallPercent = $derived(
    modelProgress.reduce((sum, m) => sum + m.percent, 0) / totalModels,
  );

  const currentModelName = $derived(
    isDownloading && currentModelIndex < totalModels
      ? MODEL_NAMES[currentModelIndex].replace("_algorithm.onnx", "")
      : null,
  );

  const needsDialog = $derived(isLoading || !allReady);
  const dialogOpen = $derived(needsDialog && !minimized);

  function minimize() {
    minimized = true;
  }

  function restore() {
    minimized = false;
  }

  async function startDownload() {
    if (isDownloading)
      return;

    isDownloading = true;
    error = null;
    currentModelIndex = 0;

    const statusData = statusQuery.data;
    const modelsToDownload = statusData
      ? MODEL_NAMES.filter(
        (_, i) => !statusData.models[i]?.exists,
      )
      : MODEL_NAMES;

    modelProgress = MODEL_NAMES.map((name, i) => ({
      name,
      percent: statusData?.models[i]?.exists ? 100 : 0,
      downloadedBytes: 0,
      totalBytes: 0,
    }));

    const unlisten = await listen<DownloadProgress>(
      "model-download-progress",
      (event) => {
        const idx = MODEL_NAMES.indexOf(event.payload.model_name);
        if (idx >= 0) {
          modelProgress[idx] = {
            name: event.payload.model_name,
            percent: event.payload.progress_percent,
            downloadedBytes: event.payload.downloaded_bytes,
            totalBytes: event.payload.total_bytes,
          };
        }
      },
    );

    try {
      for (let i = 0; i < modelsToDownload.length; i++) {
        const modelName = modelsToDownload[i];
        currentModelIndex = MODEL_NAMES.indexOf(modelName);

        await downloadMutation.mutateAsync(modelName);
      }

      await queryClient.invalidateQueries({ queryKey: ["models-status"] });
    }
    catch (e) {
      error = e instanceof Error ? e.message : String(e);
    }
    finally {
      unlisten();
      isDownloading = false;
    }
  }

  return {
    get allReady() {
      return allReady;
    },
    get isLoading() {
      return isLoading;
    },
    get isDownloading() {
      return isDownloading;
    },
    get modelProgress() {
      return modelProgress;
    },
    get overallPercent() {
      return overallPercent;
    },
    get currentModelName() {
      return currentModelName;
    },
    get totalModels() {
      return totalModels;
    },
    get error() {
      return error;
    },
    get statusData(): ModelsCheckResult | undefined {
      return statusQuery.data;
    },
    get minimized() {
      return minimized;
    },
    get needsDialog() {
      return needsDialog;
    },
    get dialogOpen() {
      return dialogOpen;
    },
    startDownload,
    minimize,
    restore,
  };
}
