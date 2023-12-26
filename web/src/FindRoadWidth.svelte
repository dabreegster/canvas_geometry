<script lang="ts">
  import SplitComponent from "./SplitComponent.svelte";
  import { map, mode } from "./stores";

  let out = JSON.parse($map!.findRoadWidth($mode.road.properties.id));
  console.log(out);
</script>

<SplitComponent>
  <div slot="sidebar">
    <p>Finding width of this road...</p>
    <div>
      <button on:click={() => mode.set({ mode: "neutral" })}>Back</button>
    </div>
  </div>
  <g slot="map">
    {#each out.test_points as test}
      <circle cx={test.pt.x} cy={test.pt.y} r="0.2" />
      <line
        class="left"
        x1={test.line_left.start.x}
        y1={test.line_left.start.y}
        x2={test.line_left.end.x}
        y2={test.line_left.end.y}
      />
      <line
        class="right"
        x1={test.line_right.start.x}
        y1={test.line_right.start.y}
        x2={test.line_right.end.x}
        y2={test.line_right.end.y}
      />
    {/each}
  </g>
</SplitComponent>

<style>
  circle {
    fill: cyan;
  }

  .left {
    stroke: red;
    stroke-width: 0.1;
  }

  .right {
    stroke: blue;
    stroke-width: 0.1;
  }
</style>
