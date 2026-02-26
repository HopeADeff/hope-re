<script lang="ts">
  import type { WithChild } from "bits-ui";
  import type { HTMLAttributes } from "svelte/elements";

  import { box, mergeProps } from "svelte-toolbelt";

  import { useFileDropZoneTextarea } from "./file-drop-zone.svelte.js";

  type Props = HTMLAttributes<HTMLTextAreaElement>;

  const { onpaste, ondragover, ondrop, child, ...rest }: WithChild & Props = $props();

  const fileDropZoneTextareaState = useFileDropZoneTextarea({
    onpaste: box.with(() => onpaste),
    ondragover: box.with(() => ondragover),
    ondrop: box.with(() => ondrop),
  });

  const mergedProps = $derived(mergeProps(fileDropZoneTextareaState.props, rest));
</script>

{#if child}
  {@render child({ props: mergedProps })}
{:else}
  <textarea {...mergedProps}></textarea>
{/if}
