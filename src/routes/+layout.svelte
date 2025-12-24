<script lang="ts">
  import favicon from "$lib/assets/favicon.svg";

  import "../app.css";

  const { children } = $props();
</script>

<svelte:head>
  <link rel="icon" href={favicon} />
</svelte:head>

{@render children()}
