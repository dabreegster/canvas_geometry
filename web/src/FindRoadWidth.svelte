<script lang="ts">
  import { along, length } from "@turf/turf";
  import { mode } from "./stores";

  let linestring = $mode.road;
  let pointsOnLine = findPointsOnLine(1000.0);

  // For each point, create a perpendicular projection left and right

  function findPointsOnLine(stepSizeMeters: number) {
    // TODO ahhh turf assumes WGS84, but we've already projected to mercator and are in meters.
    let len = length(linestring);
    //console.log(len);
    let pts = [];
    for (let x = 0.0; x < len; x += stepSizeMeters) {
      pts.push(along(linestring, x));
    }
    // TODO Imprecise towards the end, so don't forget the last point
    //console.log(pts);
    return pts;
  }
</script>

{#each pointsOnLine as pt}
  <circle
    cx={pt.geometry.coordinates[0]}
    cy={pt.geometry.coordinates[1]}
    r="3"
  />
{/each}
<circle cx="10" cy="10" r="20" />

<style>
  circle {
    fill: cyan;
  }
</style>
