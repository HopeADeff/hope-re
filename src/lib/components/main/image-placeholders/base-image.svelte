<script lang="ts">
  import { ImageIcon, UploadIcon } from "@lucide/svelte";
  import { Badge } from "$lib/components/ui/badge";
  import {
    ACCEPT_IMAGE,
    Root as FileDropZone,
    Trigger as FileDropZoneTrigger,
  } from "$lib/components/ui/file-drop-zone";
  import { cn } from "$lib/utils";
  import { toast } from "svelte-sonner";

  import type { BaseImagePlaceholderProps } from "./types";

  const {
    imageSrc = null,
    label,
    onUpload,
    readonly = false,
    children,
  }: BaseImagePlaceholderProps = $props();

  function handleFileRejected({ reason, file }: { reason: string; file: File }) {
    toast.error(`${file.name}: ${reason}`);
  }

  const labelColour = $derived<string>(
    readonly ? "text-emerald-600 dark:text-emerald-400" : "text-indigo-600 dark:text-indigo-400",
  );

  const labelBgColour = $derived<string>(
    readonly ? "bg-emerald-500/10" : "bg-indigo-500/10",
  );
</script>

<div class="flex flex-col h-full">
  <div class="mb-3 flex items-center gap-2">
    <div class={cn("p-1.5 rounded-lg", labelBgColour)}>
      <ImageIcon class={cn("size-4", labelColour)} />
    </div>
    <h3 class={cn("text-sm font-bold", labelColour)}>
      {label}
    </h3>
    {#if imageSrc}
      <Badge variant={readonly ? "default" : "secondary"} class="ml-auto">
        <span class="text-xs">Loaded</span>
      </Badge>
    {/if}
  </div>

  {#if !readonly && onUpload}
    <FileDropZone onUpload={onUpload}
                  maxFiles={1}
                  fileCount={0}
                  accept={ACCEPT_IMAGE}
                  onFileRejected={handleFileRejected}>
      <FileDropZoneTrigger class="flex-1">
        <div class={cn(
          "relative flex-1 rounded-xl border overflow-hidden bg-muted/30 min-h-[300px] md:min-h-[400px] transition-colors",
          !imageSrc && "border-dashed cursor-pointer border-indigo-300/50 dark:border-indigo-500/30",
          imageSrc && "border-indigo-200/50 dark:border-indigo-500/20",
        )}>
          {#if imageSrc}
            <div class="absolute inset-0 flex items-center justify-center p-4">
              <div class="absolute inset-0 opacity-20"
                   style="
                     background-image:
                     linear-gradient(45deg, #ccc 25%, transparent 25%),
                     linear-gradient(-45deg, #ccc 25%, transparent 25%),
                     linear-gradient(45deg, transparent 75%, #ccc 75%),
                     linear-gradient(-45deg, transparent 75%, #ccc 75%);
                     background-size: 20px 20px;
                     background-position: 0 0, 0 10px, 10px -10px, -10px 0px;
                   "
              ></div>

              <img src={imageSrc}
                   alt={label}
                   class="relative z-10 max-w-full max-h-full object-contain rounded-lg" />
            </div>
          {:else}
            <div class="absolute inset-0 flex flex-col items-center justify-center p-6 text-center">
              <div class="p-4 rounded-lg mb-3 bg-indigo-500/10">
                <UploadIcon class="size-10 text-indigo-600 dark:text-indigo-400" />
              </div>
              <p class="text-sm font-medium mb-1">
                Upload your image
              </p>
              <p class="text-xs text-muted-foreground mb-4">
                <span class="hidden md:inline">Drag & drop or click to browse</span>
                <span class="md:hidden">Tap to browse image</span>
              </p>
              <div class="flex gap-2 mt-2">
                <Badge variant="secondary" class="text-xs">JPG</Badge>
                <Badge variant="secondary" class="text-xs">PNG</Badge>
                <Badge variant="secondary" class="text-xs">WEBP</Badge>
              </div>
            </div>
          {/if}
        </div>
      </FileDropZoneTrigger>
    </FileDropZone>
  {:else}
    <div class={cn(
      "relative flex-1 rounded-xl border overflow-hidden bg-muted/30 min-h-[300px] md:min-h-[400px] transition-colors",
      !imageSrc && "border-dashed border-emerald-300/50 dark:border-emerald-500/30",
      imageSrc && "border-emerald-200/50 dark:border-emerald-500/20",
    )}>
      {#if imageSrc}
        <div class="absolute inset-0 flex items-center justify-center p-4">
          <div class="absolute inset-0 opacity-20"
               style="
                 background-image:
                 linear-gradient(45deg, #ccc 25%, transparent 25%),
                 linear-gradient(-45deg, #ccc 25%, transparent 25%),
                 linear-gradient(45deg, transparent 75%, #ccc 75%),
                 linear-gradient(-45deg, transparent 75%, #ccc 75%);
                 background-size: 20px 20px;
                 background-position: 0 0, 0 10px, 10px -10px, -10px 0px;
               "
          ></div>

          <img src={imageSrc}
               alt={label}
               class="relative z-10 max-w-full max-h-full object-contain rounded-lg" />
        </div>

        {@render children?.()}
      {:else}
        <div class="absolute inset-0 flex flex-col items-center justify-center p-6 text-center">
          <div class="p-4 rounded-lg mb-3 bg-emerald-500/10">
            <ImageIcon class="size-10 text-emerald-600 dark:text-emerald-400" />
          </div>
          <p class="text-sm font-medium text-muted-foreground mb-1">
            No rendered image yet
          </p>
          <p class="text-xs text-muted-foreground/70">
            Process an image to see the result here
          </p>
        </div>
      {/if}
    </div>
  {/if}
</div>
