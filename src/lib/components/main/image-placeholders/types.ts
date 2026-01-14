import type { Snippet } from "svelte";

export type BaseImagePlaceholderProps = {
  files?: FileList | null;
  elementRef?: HTMLInputElement;
  imageSrc?: string | null;
  label: string;
  onFileSelect?: (file: FileList) => void;
  readonly?: boolean;
  children?: Snippet | null;
};

export type ImageFullscreenDialogProps = {
  open?: boolean;
  imageSrc: string | null;
  onOpenChange?: (open: boolean) => void;
};

export type RenderedImageActionsProps = {
  onDownload?: () => void;
  onFullscreen?: () => void;
};
