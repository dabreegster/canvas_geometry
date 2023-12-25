<script lang="ts">
  import { polygonToSvg } from "./math";
  import { map, mode } from "./stores";

  let out = JSON.parse($map!.findIntersectionGeometry($mode.intersection.properties.id));
  console.log($mode.intersection);
  console.log(out);
</script>

{#each out.thick_roads as p}
  <polygon points={polygonToSvg(p)} />
{/each}

<style>
  polygon {
    fill: green;
    fill-opacity: 0.5;
  }
  polygon:hover {
    fill-opacity: 0.9;
  }
</style>
