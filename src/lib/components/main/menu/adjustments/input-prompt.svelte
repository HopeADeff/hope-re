<script lang="ts">
  import { MessageSquareIcon } from "@lucide/svelte";
  import { Badge } from "$lib/components/ui/badge";
  import { Textarea } from "$lib/components/ui/textarea";

  type InputPromptProps = {
    value?: string;
  };

  let { value = $bindable("") }: InputPromptProps = $props();

  const textareaId = `input-prompt-${Math.random().toString(36).substring(2, 9)}`;

  const charCount = $derived(value.length);
  const maxChars = 500;
</script>

<div class="space-y-3">
  <div class="flex items-center justify-between">
    <div class="flex items-center gap-2">
      <div class="p-1.5 rounded-lg bg-cyan-500/10">
        <MessageSquareIcon class="size-4 text-cyan-500" />
      </div>
      <label for={textareaId} class="text-sm font-semibold">Input Prompt</label>
    </div>
    <Badge variant="secondary" class="text-xs font-mono">
      {charCount}/{maxChars}
    </Badge>
  </div>

  <Textarea id={textareaId}
            bind:value
            placeholder="Enter a custom prompt to guide the protection..."
            class="min-h-[100px] resize-none"
            maxlength={maxChars} />

  <p class="text-xs text-muted-foreground">
    Provide a custom text prompt to guide how the protection is applied.
  </p>
</div>
