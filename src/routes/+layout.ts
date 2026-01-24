import { QueryClient } from "@tanstack/svelte-query";
import { browser } from "$app/environment";

export const prerender = true;
export const ssr = false;

export async function load() {
  const queryClient = new QueryClient({
    defaultOptions: {
      queries: {
        enabled: browser,
      },
    },
  });

  return { queryClient };
}
