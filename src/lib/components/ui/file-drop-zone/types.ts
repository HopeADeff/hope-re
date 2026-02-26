/* eslint-disable ts/no-empty-object-type */
import type { WithChildren } from "bits-ui";
import type { WithElementRef } from "svelte-toolbelt";
import type { HTMLInputAttributes, HTMLLabelAttributes } from "svelte/elements";

export type FileRejectedReason
  = | "Maximum file size exceeded"
    | "File type not allowed"
    | "Maximum files uploaded";

export type FileDropZoneRootPropsWithoutHTML = WithChildren<{
  ref?: HTMLInputElement | null;
  id?: string;
  /**
   *
   *
   * @param files
   */
  onUpload: (files: File[]) => Promise<void>;
  maxFiles?: number;
  fileCount?: number;
  maxFileSize?: number;
  onFileRejected?: (opts: { reason: FileRejectedReason; file: File }) => void;
  accept?: string;
}>;

export type FileDropZoneRootProps = FileDropZoneRootPropsWithoutHTML
  & Omit<HTMLInputAttributes, "multiple" | "files" | "id" | "class">;

export type FileDropZoneTriggerPropsWithoutHTML = WithChildren<WithElementRef<{}>>;

export type FileDropZoneTriggerProps = FileDropZoneTriggerPropsWithoutHTML
  & Omit<HTMLLabelAttributes, "for">;
