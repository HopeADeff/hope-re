import type { Update } from "@tauri-apps/plugin-updater";

import { createQuery } from "@tanstack/svelte-query";
import { check } from "@tauri-apps/plugin-updater";

export function useCheckForUpdate() {
  return createQuery(() => ({
    queryKey: ["updater-check"],
    queryFn: async (): Promise<Update | null> => await check(),
    staleTime: 5 * 60 * 1000,
    gcTime: 10 * 60 * 1000,
    retry: 1,
  }));
}
