<script lang="ts">
  import { buffer } from "@turf/turf";
  import { gjToSvg, mercatorToSpherical, sphericalToMercator } from "./math";
  import { mode } from "./stores";

  export let gj;

  // TODO Offline and losing my sanity
  function has(list, x) {
    for (let y of list) {
      if (x == y) {
        return true;
      }
    }
    return false;
  }

  let roads = gj.features
    .filter(
      (f) =>
        f.geometry.type == "LineString" &&
        has($mode.intersection.properties.roads, f.properties.id)
    )
    .map((f) => mercatorToSpherical(gj, f));

  let buffered = roads.map((f) => sphericalToMercator(gj, buffer(f, 0.005)));
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
