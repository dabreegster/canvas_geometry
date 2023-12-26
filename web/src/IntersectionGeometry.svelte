<script lang="ts">
  import { polygonToSvg } from "./math";
  import SplitComponent from "./SplitComponent.svelte";
  import { map, mode } from "./stores";

  $: out = JSON.parse(
    $map!.findIntersectionGeometry($mode.intersection.properties.id)
  );

  let showThickRoads = true;
  let showOverlaps = true;
  let showUnioned = true;
</script>

<SplitComponent>
  <div slot="sidebar">
    <p>Intersection geometry</p>
    <div>
      <button on:click={() => mode.set({ mode: "neutral" })}>Back</button>
    </div>
    <div>
      <input type="checkbox" bind:checked={showThickRoads} />Show thick roads ({out
        .thick_roads.length})
    </div>
    <div>
      <input type="checkbox" bind:checked={showOverlaps} />Show overlaps ({out.overlaps.flat()
        .length})
    </div>
    <div>
      <input type="checkbox" bind:checked={showUnioned} />Show unioned ({out.unioned.flat()
        .length})
    </div>
  </div>
  <g slot="map">
    {#if showThickRoads}
      {#each out.thick_roads as p}
        <polygon points={polygonToSvg(p)} class="thick" />
      {/each}
    {/if}
    {#if showOverlaps}
      {#each out.overlaps.flat() as p}
        <polygon points={polygonToSvg(p)} class="overlaps" />
      {/each}
    {/if}
    {#if showUnioned}
      {#each out.unioned.flat() as p}
        <polygon points={polygonToSvg(p)} class="unioned" />
      {/each}
    {/if}
  </g>
</SplitComponent>

<style>
  polygon {
    fill-opacity: 0.5;
  }
  polygon:hover {
    fill-opacity: 0.9;
  }

  .thick {
    fill: green;
  }
  .overlaps {
    fill: red;
  }
  .unioned {
    fill: blue;
  }
</style>
