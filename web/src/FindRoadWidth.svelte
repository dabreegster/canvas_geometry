<script lang="ts">
  import { along, length } from "@turf/turf";
  import { mercatorToSpherical, sphericalToMercator } from "./math";
  import { mode } from "./stores";

  export let gj;

  let linestring = mercatorToSpherical(gj, $mode.road);
  let pointsOnLine = findPointsOnLine(1.0);

  // For each point, create a perpendicular projection left and right

  function findPointsOnLine(stepSizeMeters: number) {
    let stepSizeKm = stepSizeMeters / 1000.0;
    let len = length(linestring);
    let pts = [];
    for (let x = 0.0; x < len; x += stepSizeKm) {
      pts.push(sphericalToMercator(gj, along(linestring, x)));
    }
    // TODO Imprecise towards the end, so don't forget the last point
    return pts;
  }
</script>

{#each pointsOnLine as pt}
  <circle
    cx={pt.geometry.coordinates[0]}
    cy={pt.geometry.coordinates[1]}
    r="0.2"
  />
{/each}

<style>
  circle {
    fill: cyan;
  }
</style>
