<script lang="ts">
  import Canvas from "./Canvas.svelte";
  import ExampleLoader from "./ExampleLoader.svelte";
  import FindRoadWidth from "./FindRoadWidth.svelte";
  import IntersectionGeometry from "./IntersectionGeometry.svelte";
  import Layout from "./Layout.svelte";
  import Neutral from "./Neutral.svelte";
  import {
    map,
    mapContents,
    mode,
    showRealRoadWidth,
    sidebarContents,
  } from "./stores";

  let sidebarDiv;
  $: if (sidebarDiv && $sidebarContents) {
    sidebarDiv.innerHTML = "";
    sidebarDiv.appendChild($sidebarContents);
  }
</script>

<Layout>
  <div slot="left">
    <ExampleLoader />
    <div>
      <label>
        <input type="checkbox" bind:checked={$showRealRoadWidth} /> Show calculated
        road widths
      </label>
    </div>
    <div bind:this={sidebarDiv} />
  </div>
  <div slot="main" style="position:relative; width: 100%; height: 100vh;">
    {#key $map}
      {#if $map}
        <Canvas gj={JSON.parse($map.render())} />
      {/if}
    {/key}
  </div>
</Layout>
{#if $mode.mode == "neutral"}
  <Neutral />
{:else if $mode.mode == "find-width"}
  <FindRoadWidth />
{:else if $mode.mode == "intersection-geometry"}
  <IntersectionGeometry />
{/if}
