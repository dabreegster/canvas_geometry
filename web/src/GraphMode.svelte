<script lang="ts">
  import { linestringToSvg } from "./math";
  import SplitComponent from "./SplitComponent.svelte";
  import { map, mode } from "./stores";

  let out = JSON.parse($map!.renderGraph());
  let keepLastPoint = false;

  function traceLoop(node: number) {
    $map!.traceGraphLoop(node, keepLastPoint);
    out = JSON.parse($map!.renderGraph());
  }

  function undo() {
    $map!.undoGraph();
    out = JSON.parse($map!.renderGraph());
  }
</script>

<SplitComponent>
  <div slot="sidebar">
    <p>Graph mode</p>
    <div>
      <button on:click={() => mode.set({ mode: "neutral" })}>Back</button>
    </div>
    <div>
      <button on:click={undo}>Undo</button>
    </div>
    <div>
      <input type="checkbox" bind:checked={keepLastPoint} />Keep last point
    </div>
  </div>
  <g slot="map">
    {#each Object.values(out.edges) as edge}
      <polyline points={linestringToSvg(edge.linestring)}>
        <title>{JSON.stringify(edge)}</title>
      </polyline>
    {/each}
    {#each Object.values(out.nodes) as node}
      <circle
        cx={node.point.x}
        cy={node.point.y}
        r="1"
        on:click={() => traceLoop(node.id)}
      >
        <title>{JSON.stringify(node)}</title>
      </circle>
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
    /* Because there's no z-indexing... */
    stroke: blue;
    stroke-width: 1.5;
  }

  circle {
    fill: red;
  }
  circle:hover {
    fill: blue;
  }
</style>
