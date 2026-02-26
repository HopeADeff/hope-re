<script lang="ts">
  import { TargetIcon } from "@lucide/svelte";
  import * as Select from "$lib/components/ui/select";
  import { glazeStyles } from "$lib/constants";

  import type { GlazeStyleSelectProps } from "../types";

  let { value = $bindable("abstract") }: GlazeStyleSelectProps = $props();

  const contentTrigger = $derived(
    glazeStyles.find(style => style.value === value)?.label ?? "Select a style",
  );
</script>

<div class="space-y-3">
  <div class="flex items-center gap-2">
    <div class="p-1.5 rounded-lg bg-amber-500/10">
      <TargetIcon class="size-4 text-amber-600 dark:text-amber-400" />
    </div>
    <span class="text-sm font-medium">Target Style</span>
  </div>

  <Select.Root type="single" bind:value>
    <Select.Trigger class="w-full hover:border-foreground/20 transition-colors" aria-label="Glaze style selection">
      {contentTrigger}
    </Select.Trigger>
    <Select.Content>
      {#each glazeStyles as style}
        <Select.Item value={style.value}>
          <div class="flex items-center gap-3 py-1">
            <span class="text-base">{style.emoji}</span>
            <div class="flex flex-col gap-0.5">
              <span class="font-medium text-sm">{style.label}</span>
              <span class="text-xs text-muted-foreground">{style.description}</span>
            </div>
          </div>
        </Select.Item>
      {/each}
    </Select.Content>
  </Select.Root>

  <p class="text-xs text-muted-foreground">
    Your image will be shifted toward this art style to confuse AI scrapers.
  </p>
</div>
