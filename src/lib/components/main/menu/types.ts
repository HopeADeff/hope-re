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
  glazeStyle?: "abstract" | "impressionist" | "cubist" | "sketch" | "watercolor";
  nightshadeTarget?: "dog" | "cat" | "car" | "landscape" | "person" | "building" | "food" | "abstract";
  intensity?: number[];
  outputQuality?: number[];
  renderQuality?: number[];
  progressMessage: ProtectionProgressProps["message"];
};

// Adjustments / Setting Components types
export type AlgorithmSelectProps = {
  value?: ProtectionMenuProps["algorithm"];
};

export type GlazeStyleSelectProps = {
  value?: ProtectionMenuProps["glazeStyle"];
};

export type NightshadeTargetSelectProps = {
  value?: ProtectionMenuProps["nightshadeTarget"];
};

export type IntensitySliderProps = {
  value?: ProtectionMenuProps["intensity"];
};

export type OutputQualitySliderProps = {
  value?: ProtectionMenuProps["outputQuality"];
};

export type RenderQualitySliderProps = {
  value?: ProtectionMenuProps["renderQuality"];
};
