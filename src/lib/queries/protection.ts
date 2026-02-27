import type { ProtectionMenuProps } from "$lib/components";

import { createMutation } from "@tanstack/svelte-query";
import { invoke } from "@tauri-apps/api/core";

export type ProtectionSettings = {
  algorithm: "noise" | "glaze" | "nightshade";
  intensity: number;
  output_quality: number;
  render_quality: number;
  glaze_style?: string;
  nightshade_target?: string;
};

export type ProtectionResult = {
  image_base64: string;
  success: boolean;
  message: string;
};

export type ProtectImageInput = {
  imageBase64: string;
  settings: ProtectionSettings;
};

export function buildProtectionSettings(params: {
  algorithm: ProtectionMenuProps["algorithm"];
  glazeStyle: ProtectionMenuProps["glazeStyle"];
  nightshadeTarget: ProtectionMenuProps["nightshadeTarget"];
  intensity: number[];
  outputQuality: number[];
  renderQuality: number[];
}): ProtectionSettings {
  const selectedAlgorithm = params.algorithm ?? "noise";
  return {
    algorithm: selectedAlgorithm,
    intensity: params.intensity[0] / 100,
    output_quality: params.outputQuality[0],
    render_quality: params.renderQuality[0],
    glaze_style: selectedAlgorithm === "glaze" ? params.glazeStyle : undefined,
    nightshade_target: selectedAlgorithm === "nightshade" ? params.nightshadeTarget : undefined,
  };
}

export async function protectImage(
  imageBase64: string,
  settings: ProtectionSettings,
): Promise<ProtectionResult> {
  return await invoke<ProtectionResult>("protect_image", {
    imageBase64,
    settings,
  });
}

export function useProtectImage() {
  return createMutation(() => ({
    mutationFn: async (input: ProtectImageInput) =>
      await protectImage(input.imageBase64, input.settings),
  }));
}
