import type { Snippet } from "svelte";

export type BaseImagePlaceholderProps = {
  imageSrc?: string | null;
  label: string;
  onUpload?: (files: File[]) => Promise<void>;
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
