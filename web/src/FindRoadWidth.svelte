<script lang="ts">
  import { map, mode } from "./stores";

  let out = JSON.parse($map!.findRoadWidth($mode.road.properties.id));
  console.log(out);
</script>

{#each out.test_points as pt}
  <circle cx={pt.x} cy={pt.y} r="0.2" />
{/each}

<style>
  circle {
    fill: cyan;
  }
</style>
