import { createMutation, createQuery } from "@tanstack/svelte-query";
import { invoke } from "@tauri-apps/api/core";

export type ModelStatus = {
  name: string;
  exists: boolean;
  size_bytes: number;
};

export type ModelsCheckResult = {
  models: ModelStatus[];
  all_ready: boolean;
  models_dir: string;
};

export function useModelsStatus() {
  return createQuery(() => ({
    queryKey: ["models-status"],
    queryFn: async () =>
      await invoke<ModelsCheckResult>("check_models_status"),
    staleTime: 30 * 1000,
    gcTime: 60 * 1000,
    retry: 1,
  }));
}

export function useDownloadModel() {
  return createMutation(() => ({
    mutationFn: async (modelName: string) =>
      await invoke<string>("download_model", { modelName }),
  }));
}
