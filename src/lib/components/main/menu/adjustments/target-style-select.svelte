<script lang="ts">
  import { TargetIcon } from "@lucide/svelte";
  import * as Select from "$lib/components/ui/select";
  import { targetStyles } from "$lib/constants";

  import type { TargetStyleSelectProps } from "../types";

  let { value = $bindable("abstract") }: TargetStyleSelectProps = $props();

  const contentTrigger = $derived(
    targetStyles.find(style => style.value === value)?.label ?? "Select a style",
  );
</script>

<div class="space-y-3">
  <div class="flex items-center gap-2">
    <div class="p-1.5 rounded-lg bg-pink-500/10">
      <TargetIcon class="size-4 text-pink-500" />
    </div>
    <span class="text-sm font-semibold">Target Style</span>
  </div>

  <Select.Root type="single" bind:value>
    <Select.Trigger class="w-full hover:border-pink-300 transition-colors" aria-label="Target style selection">
      {contentTrigger}
    </Select.Trigger>
    <Select.Content>
      {#each targetStyles as style}
        <Select.Item value={style.value}>
          <div class="flex items-center gap-3 py-1">
            <span class="text-lg">{style.emoji}</span>
            <span class="font-medium text-sm">{style.label}</span>
          </div>
        </Select.Item>
      {/each}
    </Select.Content>
  </Select.Root>
</div>
