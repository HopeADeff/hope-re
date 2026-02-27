import { save } from "@tauri-apps/plugin-dialog";
import { writeFile } from "@tauri-apps/plugin-fs";
import { toast } from "svelte-sonner";

export function useImage() {
  let originalImage = $state<string | null>(null);
  let fullscreenOpen = $state<boolean>(false);

  async function handleUpload(files: File[]) {
    const file = files[0];
    if (!file)
      return;

    const reader = new FileReader();

    reader.onload = (e) => {
      originalImage = e.target?.result as string;
      toast.success(`Loaded ${file.name}`);
    };

    reader.onerror = () => {
      toast.error("Failed to load image");
    };

    reader.readAsDataURL(file);
  }

  async function handleDownload(renderedImage: string | null, algorithm: string) {
    if (!renderedImage)
      return;

    try {
      const timestamp = new Date().toISOString().replace(/[:.]/g, "-").slice(0, -5);
      const defaultPath = `protected-${algorithm}-${timestamp}.png`;

      const filePath = await save({
        filters: [{
          name: "Image",
          extensions: ["png"],
        }],
        defaultPath,
      });

      if (!filePath)
        return;

      const response = await fetch(renderedImage);
      const arrayBuffer = await response.arrayBuffer();
      const bytes = new Uint8Array(arrayBuffer);

      await writeFile(filePath, bytes);
      toast.success("Image saved successfully");
    }
    catch (error) {
      toast.error("Failed to save image");
      console.error("Save error:", error);
    }
  }

  function handleFullscreen() {
    fullscreenOpen = true;
  }

  function clear() {
    originalImage = null;
    fullscreenOpen = false;
  }

  return {
    get originalImage() {
      return originalImage;
    },
    get fullscreenOpen() {
      return fullscreenOpen;
    },
    set fullscreenOpen(value: boolean) {
      fullscreenOpen = value;
    },
    get hasImage() {
      return !!originalImage;
    },
    handleUpload,
    handleDownload,
    handleFullscreen,
    clear,
  };
}
