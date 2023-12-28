<script lang="ts">
  import { lineToSvg, polygonToSvg } from "./math";
  import SplitComponent from "./SplitComponent.svelte";
  import { map, mode } from "./stores";

  $: out = JSON.parse($map!.findRoadWidth($mode.road.properties.id));
</script>

<SplitComponent>
  <div slot="sidebar">
    <p>Finding width of this road...</p>
    <p>Max left: {out.max_left_width.toFixed(2)}</p>
    <p>Max right: {out.max_right_width.toFixed(2)}</p>
    <div>
      <button on:click={() => mode.set({ mode: "neutral" })}>Back</button>
    </div>
  </div>
  <g slot="map">
    {#each out.test_lines as test}
      {@const direction = test.left ? "left" : "right"}
      {#if test.hit}
        <line class={`${direction} hits`} {...lineToSvg(test.hit[0])}>
          <title>Length: {test.hit[1].toFixed(2)}</title>
        </line>
      {:else}
        <line class={direction} {...lineToSvg(test.full_line)} />
      {/if}
    {/each}

    <polygon
      points={polygonToSvg(out.buffered_polygon)}
    />
  </g>
</SplitComponent>

<style>
  line {
    stroke-width: 0.1;
  }

  .left {
    stroke: red;
  }

  .right {
    stroke: blue;
  }

  .hits {
    stroke-dasharray: 0.5;
  }
  .hits:hover {
    stroke-opacity: 0.5;
  }

  polygon {
    fill: none;
    stroke: green;
  }
</style>
