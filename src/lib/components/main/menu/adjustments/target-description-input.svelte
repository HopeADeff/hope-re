<script lang="ts">
  import { FileTextIcon } from "@lucide/svelte";
  import { Badge } from "$lib/components/ui/badge";
  import { Textarea } from "$lib/components/ui/textarea";

  import type { TargetDescriptionInputProps } from "../types";

  let { value = $bindable("") }: TargetDescriptionInputProps = $props();

  const textareaId = `target-desc-${Math.random().toString(36).substring(2, 9)}`;

  const charCount = $derived(value.length);
  const maxChars = 500;
</script>

<div class="space-y-3">
  <div class="flex items-center justify-between">
    <div class="flex items-center gap-2">
      <div class="p-1.5 rounded-lg bg-indigo-500/10">
        <FileTextIcon class="size-4 text-indigo-500" />
      </div>
      <label for={textareaId} class="text-sm font-semibold">Target Description</label>
    </div>
    <Badge variant="secondary" class="text-xs font-mono">
      {charCount}/{maxChars}
    </Badge>
  </div>

  <Textarea id={textareaId}
            bind:value
            placeholder="Describe the target style or content you want to protect against..."
            class="min-h-[100px] resize-none"
            maxlength={maxChars} />

  <p class="text-xs text-muted-foreground">
    Describe what you want to protect your image from &lpar;e.g., &quot;abstract art style&quot;, &quot;anime rendering&quot;&rpar;.
  </p>
</div>
