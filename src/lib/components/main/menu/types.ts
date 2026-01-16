// Protection Progress types
export type ProtectionProgressProps = {
  isProcessing?: boolean;
  progress?: number;
  status?: "idle" | "processing" | "success" | "error";
  message?: string;
};

// Protection Menu types
export type ProtectionMenuProps = Omit<ProtectionProgressProps, "message"> & {
  algorithm?: "noise" | "glaze" | "nightshade";
  targetStyle?: "abstract" | "impressionist" | "cubist" | "sketch" | "watercolor";
  targetDescription?: string;
  inputPrompt?: string;
  intensity?: number[];
  outputQuality?: number[];
  renderQuality?: number[];
  progressMessage: ProtectionProgressProps["message"];
};

// Adjustments / Setting types
export type AlgorithmSelectProps = {
  value: ProtectionMenuProps["algorithm"];
};

export type InputPromptProps = {
  value: ProtectionMenuProps["inputPrompt"];
};

export type IntensitySliderProps = {
  value: ProtectionMenuProps["intensity"];
};

export type OutputQualitySliderProps = {
  value: ProtectionMenuProps["outputQuality"];
};

export type RenderQualitySliderProps = {
  value: ProtectionMenuProps["renderQuality"];
};

export type TargetDescriptionInputProps = {
  value: ProtectionMenuProps["targetDescription"];
};

export type TargetStyleSelectProps = {
  value: ProtectionMenuProps["targetStyle"];
};
