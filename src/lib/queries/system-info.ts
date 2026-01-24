import { createQuery } from "@tanstack/svelte-query";
import { invoke } from "@tauri-apps/api/core";

type PlatformInfo = {
  os: string;
  hostname: string;
};

type SystemInfo = {
  platform: PlatformInfo;
  cpu: string;
  memory: string;
  gpu: string;
  storage: string;
};

export function useSystemInfo() {
  return createQuery(() => ({
    queryKey: ["system-info"],
    queryFn: async () =>
      await invoke<SystemInfo>("get_system_info"),
    staleTime: 5 * 60 * 1000,
    gcTime: 10 * 60 * 1000,
    retry: 2,
  }));
}
