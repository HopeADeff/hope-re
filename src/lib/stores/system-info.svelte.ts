import { invoke } from "@tauri-apps/api/core";

export type PlatformInfo = {
  os: string;
  hostname: string;
};

export type SystemInfo = {
  platform: PlatformInfo;
  cpu: string;
  memory: string;
  gpu: string;
  storage: string;
};

export function useSystemInfo() {
  let systemInfo = $state<SystemInfo | null>(null);
  let isLoading = $state<boolean>(false);
  let error = $state<string | null>(null);

  async function fetchSystemInfo() {
    isLoading = true;
    error = null;

    try {
      const info = await invoke<SystemInfo>("get_system_info");
      systemInfo = info;
    }
    catch (err) {
      console.error("Failed to fetch system info:", err);
      error = err instanceof Error ? err.message : "Unknown error";
    }
    finally {
      isLoading = false;
    }
  }

  function resetSystemInfo() {
    systemInfo = null;
    isLoading = false;
    error = null;
  }

  return {
    get systemInfo() { return systemInfo; },
    get isLoading() { return isLoading; },
    get error() { return error; },
    fetchSystemInfo,
    resetSystemInfo,
  };
}
