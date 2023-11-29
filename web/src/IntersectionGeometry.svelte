<script lang="ts">
  import { buffer } from "@turf/turf";
  import { mode } from "./stores";

  export let gj;

  let roads = gj.features.filter(
    (f) =>
      f.geometry.type == "LineString" &&
      f.properties.id in $mode.intersection.properties.roads
  );

  let buffered = roads.map((f) => buffer(f, 0.1));
  console.log(buffered);

  function gjToSvg(points) {
    return points.map((pt) => `${pt[0]},${pt[1]}`).join(" ");
  }
</script>

{#each buffered as f}
  <polygon points={gjToSvg(f.geometry.coordinates[0])} />
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
