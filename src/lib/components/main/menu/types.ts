export type ProtectionMenuProps = {
  algorithm?: string;
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
