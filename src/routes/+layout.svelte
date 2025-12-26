<script lang="ts">
  import type { Snippet } from "svelte";

  import { WindowTitle } from "$lib/components";

  import "../app.css";

  const { children }: { children: Snippet } = $props();
</script>

<WindowTitle />
{@render children()}
