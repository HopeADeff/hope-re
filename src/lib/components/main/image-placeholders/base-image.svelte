<script lang="ts">
  import { ImageIcon, SparklesIcon, UploadIcon } from "@lucide/svelte";
  import { Badge } from "$lib/components/ui/badge";
  import { cn } from "$lib/utils";

  import type { BaseImagePlaceholderProps } from "./types";

  let {
    files = $bindable(),
    elementRef = $bindable(),
    imageSrc = null,
    label,
    onFileSelect,
    readonly = false,
    children,
  }: BaseImagePlaceholderProps = $props();

  let isDragging = $state<boolean>(false);

  function handleDragOver(e: DragEvent) {
    if (readonly)
      return;

    e.preventDefault();
    isDragging = true;
  }

  function handleDragLeave(e: DragEvent) {
    if (readonly)
      return;

    e.preventDefault();
    isDragging = false;
  }

  function handleDrop(e: DragEvent) {
    if (readonly)
      return;

    e.preventDefault();
    isDragging = false;

    const files = e.dataTransfer?.files;
    if (files && files.length > 0 && onFileSelect) {
      onFileSelect(files);
    }
  }

  function handleClick() {
    if (readonly)
      return;
    elementRef?.click();
  }

  function handleFileInput(e: Event) {
    const target = e.target as HTMLInputElement;
    const files = target.files;

    if (files && files.length > 0 && onFileSelect) {
      onFileSelect(files);
    }
  }

  const labelColour = $derived<string>(
    readonly ? "text-emerald-600 dark:text-emerald-400" : "text-blue-600 dark:text-blue-400",
  );

  const labelBgColour = $derived<string>(
    readonly ? "bg-emerald-500/10" : "bg-blue-500/10",
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

  <!-- svelte-ignore a11y_no_noninteractive_tabindex -->
  <div class={cn(
    "relative flex-1 rounded-xl border-2 overflow-hidden bg-linear-to-br from-muted/20 to-muted/40 min-h-[300px] md:min-h-[400px] transition-all duration-300",
    (!imageSrc && !readonly) && "border-dashed cursor-pointer",
    isDragging && "border-primary bg-primary/5 shadow-lg scale-[1.02]",
    (!isDragging && !imageSrc && !readonly) && "border-blue-300",
    (!isDragging && !imageSrc && readonly) && "border-emerald-300",
    imageSrc && "border-border",
  )}
       ondragover={handleDragOver}
       ondragleave={handleDragLeave}
       ondrop={handleDrop}
       onclick={!readonly && !imageSrc ? handleClick : undefined}
       role={!readonly && !imageSrc ? "button" : undefined}
       tabindex={!readonly && !imageSrc ? 0 : undefined}
  >
    {#if (!readonly && !imageSrc)}
      <input type="file"
             bind:files
             bind:this={elementRef}
             accept="image/*"
             class="hidden"
             oninput={handleFileInput} />
    {/if}

    {#if imageSrc}
      <div class="absolute inset-0 flex items-center justify-center p-4">
        <div class="absolute inset-0 opacity-30"
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
             class="relative z-10 max-w-full max-h-full object-contain rounded-lg shadow-2xl" />
      </div>

      {@render children?.()}

    {:else}
      <div class="absolute inset-0 flex flex-col items-center justify-center p-6 text-center">
        <div class={cn(
          "p-5 rounded-2xl mb-4 transition-all duration-300",
          isDragging && "bg-primary/10 animate-bounce",
          (!isDragging && !readonly) && "bg-blue-500/10",
          (!isDragging && readonly) && "bg-emerald-500/10",
        )}>
          {#if isDragging}
            <SparklesIcon class="size-10 text-primary animate-pulse" />
          {:else if readonly}
            <ImageIcon class="size-10 text-emerald-500/70" />
          {:else}
            <UploadIcon class="size-10 text-blue-500/70" />
          {/if}
        </div>

        {#if readonly}
          <p class="text-sm font-medium text-muted-foreground mb-1">
            No rendered image yet
          </p>
          <p class="text-xs text-muted-foreground/70">
            Process an image to see the result here
          </p>
        {:else}
          <p class="text-sm font-semibold mb-1">
            {isDragging ? "Drop your image here!" : "Upload your image"}
          </p>
          <p class="text-xs text-muted-foreground mb-4">
            {isDragging ? "Release to upload" : "Drag & drop or click to browse"}
          </p>

          {#if !isDragging}
            <div class="flex gap-2 mt-2">
              <Badge variant="secondary" class="text-xs">JPG</Badge>
              <Badge variant="secondary" class="text-xs">PNG</Badge>
              <Badge variant="secondary" class="text-xs">WEBP</Badge>
            </div>
          {/if}
        {/if}
      </div>
    {/if}
  </div>
</div>
