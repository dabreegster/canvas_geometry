<script lang="ts">
  import { lineToSvg } from "./math";
  import SplitComponent from "./SplitComponent.svelte";
  import { map, mode } from "./stores";

  $: out = JSON.parse($map!.findRoadWidth($mode.road.properties.id));
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
      <line class="left" {...lineToSvg(test.line_left)} />
      <line class="right" {...lineToSvg(test.line_right)} />
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
