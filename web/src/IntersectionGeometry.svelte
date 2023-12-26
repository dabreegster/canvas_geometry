<script lang="ts">
  import { polygonToSvg } from "./math";
  import { map, mode } from "./stores";

  let out = JSON.parse(
    $map!.findIntersectionGeometry($mode.intersection.properties.id)
  );
  console.log($mode.intersection);
  console.log(out);
</script>

{#each out.thick_roads as p}
  <polygon points={polygonToSvg(p)} class="thick" />
{/each}
{#each out.overlaps.flat() as p}
  <polygon points={polygonToSvg(p)} class="overlaps" />
{/each}
{#each out.unioned.flat() as p}
  <polygon points={polygonToSvg(p)} class="unioned" />
{/each}

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
