<script lang="ts">
  import { polygonToSvg } from "./math";
  import SplitComponent from "./SplitComponent.svelte";
  import { map, mode } from "./stores";

  let out = JSON.parse(
    $map!.findIntersectionGeometry($mode.intersection.properties.id)
  );
  console.log($mode.intersection);
  console.log(out);
</script>

<SplitComponent>
  <div slot="sidebar">
    <p>Intersection geometry</p>
    <div>
      <button on:click={() => mode.set({ mode: "neutral" })}>Back</button>
    </div>
  </div>
  <g slot="map">
    {#each out.thick_roads as p}
      <polygon points={polygonToSvg(p)} class="thick" />
    {/each}
    {#each out.overlaps.flat() as p}
      <polygon points={polygonToSvg(p)} class="overlaps" />
    {/each}
    {#each out.unioned.flat() as p}
      <polygon points={polygonToSvg(p)} class="unioned" />
    {/each}
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
