<script lang="ts">
  import svgPanZoom from "svg-pan-zoom";
  import FindRoadWidth from "./FindRoadWidth.svelte";
  import { mode } from "./stores";

  export let gj;
  // TODO Hovered would be nicer
  // TODO Clicking background show unset this
  export let clickedFeature;

  let roads = gj.features.filter((f) => f.geometry.type == "LineString");
  let intersections = gj.features.filter((f) => f.geometry.type == "Point");
  let buildings = gj.features.filter((f) => f.geometry.type == "Polygon");

  function gjToSvg(points) {
    return points.map((pt) => `${pt[0]},${pt[1]}`).join(" ");
  }

  // TODO The #key is necessary to show newly rendered elements, but it's buggy
  // and a hack
  function panZoom(element) {
    svgPanZoom(element, {
      minZoom: 0.1,
      maxZoom: 50,
      zoomScaleSensitivity: 0.5,
    });
  }

  function setFocus(f) {
    mode.set({ mode: "neutral" });
    clickedFeature = f;
  }
</script>

{#key $mode}
  <svg use:panZoom>
    {#each roads as f}
      <polyline
        points={gjToSvg(f.geometry.coordinates)}
        on:click={() => setFocus(f)}
        class:clicked={clickedFeature == f}
      />
    {/each}
    {#each intersections as f}
      <circle
        cx={f.geometry.coordinates[0]}
        cy={f.geometry.coordinates[1]}
        r="1"
        on:click={() => setFocus(f)}
        class:clicked={clickedFeature == f}
      />
    {/each}
    {#each buildings as f}
      <polygon
        points={gjToSvg(f.geometry.coordinates[0])}
        on:click={() => setFocus(f)}
        class:clicked={clickedFeature == f}
      />
    {/each}
    {#if $mode.mode == "find-width"}
      <FindRoadWidth />
    {/if}
  </svg>
{/key}

<style>
  svg {
    width: 100%;
    height: 100%;
    background-color: grey;
  }

  polyline {
    fill: none;
    stroke: black;
    stroke-width: 1;
  }
  polyline:hover {
    stroke: blue;
  }

  polygon {
    fill: red;
    stroke: black;
  }
  polygon:hover {
    fill: blue;
  }

  circle {
    fill: red;
  }
  circle:hover {
    fill: blue;
  }

  .clicked {
    stroke: yellow;
    stroke-width: 3;
  }
</style>
