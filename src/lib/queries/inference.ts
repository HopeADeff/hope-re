import { createQuery } from "@tanstack/svelte-query";
import { invoke } from "@tauri-apps/api/core";

export type InferenceCapabilities = {
  providers: ExecutionProviderInfo[];
  platform: string;
};

export type ExecutionProviderInfo = {
  name: string;
};

export function useInferenceCapabilities() {
  return createQuery(() => ({
    queryKey: ["inference-capabilities"],
    queryFn: async () =>
      await invoke<InferenceCapabilities>("get_inference_capabilities"),
    staleTime: 5 * 60 * 1000,
    gcTime: 10 * 60 * 1000,
    retry: 2,
  }));
}
