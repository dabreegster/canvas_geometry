<script lang="ts">
  import { mapContents, sidebarContents } from "./stores";

  // The map slot must be in a <g> element, not <div>!
</script>

<div bind:this={$sidebarContents}>
  <slot name="sidebar" />
</div>
<g bind:this={$mapContents}>
  <slot name="map" />
</g>
