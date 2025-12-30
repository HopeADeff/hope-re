import { LazyStore } from "@tauri-apps/plugin-store";

export type Theme = "light" | "dark" | "system";

export function createTheme() {
  const store = new LazyStore("settings.json");
  let theme = $state<Theme>("system");

  async function initTheme() {
    try {
      const savedTheme = (await store.get<Theme>("theme")) || "system";
      theme = savedTheme;

      applyTheme(savedTheme);
    }
    catch (error) {
      console.error("Failed to load theme:", error);

      theme = "system";
      applyTheme("system");
    }
  }

  function getTheme() {
    return theme;
  }

  async function setTheme(newTheme: Theme) {
    try {
      theme = newTheme;
      await store.set("theme", newTheme);

      applyTheme(newTheme);
    }
    catch (error) {
      console.error("Failed to save theme:", error);
    }
  }

  function applyTheme(themeValue: Theme) {
    if (typeof document === "undefined")
      return;

    const root = document.documentElement;

    if (themeValue === "system") {
      const systemTheme = window.matchMedia("(prefers-color-scheme: dark)")
        .matches
          ? "dark"
          : "light";

      root.classList.toggle("dark", systemTheme === "dark");
    }
    else {
      root.classList.toggle("dark", themeValue === "dark");
    }
  }

  if (typeof window !== "undefined") {
    window
      .matchMedia("(prefers-color-scheme: dark)")
      .addEventListener("change", () => {
        if (theme === "system") {
          applyTheme("system");
        }
      });
  }

  return {
    initTheme,
    getTheme,
    setTheme,
    applyTheme,
  };
}

export const themeSetting = createTheme();
