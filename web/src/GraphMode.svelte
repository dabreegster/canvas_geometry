<script lang="ts">
  import { linestringToSvg } from "./math";
  import SplitComponent from "./SplitComponent.svelte";
  import { map, mode } from "./stores";

  $: out = JSON.parse($map!.renderGraph());

  function traceLoop(node: number) {}
</script>

<SplitComponent>
  <div slot="sidebar">
    <p>Graph mode</p>
    <div>
      <button on:click={() => mode.set({ mode: "neutral" })}>Back</button>
    </div>
  </div>
  <g slot="map">
    {#each Object.values(out.edges) as edge}
      <polyline points={linestringToSvg(edge.linestring)} />
    {/each}
    {#each Object.values(out.nodes) as node}
      <circle
        cx={node.point.x}
        cy={node.point.y}
        r="1"
        on:click={() => traceLoop(node.id)}
      />
    {/each}
  </g>
</SplitComponent>

<style>
  polyline {
    fill: none;
    stroke: black;
    stroke-width: 1;
  }
  polyline:hover {
    stroke: blue;
  }

  circle {
    fill: red;
  }
  circle:hover {
    fill: blue;
  }
</style>
