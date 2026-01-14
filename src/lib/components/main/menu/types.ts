// Adjustments Components Props
export type AlgorithmSelectProps = {
  value?: "noise" | "glaze" | "nightshade";
};

export type InputPromptProps = {
  value?: string;
};

export type IntensitySliderProps = {
  value?: number[];
};

export type OutputQualitySliderProps = {
  value?: number[];
};

export type RenderQualitySliderProps = {
  value?: number[];
};

export type TargetDescriptionInputProps = {
  value?: string;
};

export type TargetStyleSelectProps = {
  value?: string;
};

// Progress Bar Props
export type ProtectionProgressProps = {
  isProcessing?: boolean;
  progress?: number;
  status?: "idle" | "processing" | "success" | "error";
  message?: string;
};

// Menu Props
export type ProtectionMenuProps = {
  algorithm?: "noise" | "glaze" | "nightshade";
  targetStyle?: string;
  targetDescription?: string;
  inputPrompt?: string;
  intensity?: number[];
  outputQuality?: number[];
  renderQuality?: number[];
  isProcessing?: boolean;
  progress?: number;
  status?: "idle" | "processing" | "success" | "error";
  progressMessage?: string;
};
