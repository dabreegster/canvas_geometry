<script lang="ts">
  import svgPanZoom from "svg-pan-zoom";
  import { gjToSvg, polygonToSvg } from "./math";
  import {
    clickedFeature,
    mapContents,
    mode,
    showRealRoadWidth,
  } from "./stores";

  export let gj;
  // TODO Clicking background show unset clickedFeature

  let roads = gj.features.filter((f) => f.geometry.type == "LineString");
  let intersections = gj.features.filter((f) => f.geometry.type == "Point");
  let buildings = gj.features.filter((f) => f.geometry.type == "Polygon");

  // TODO The #key is necessary to show newly rendered elements, but it's buggy
  // and a hack
  let prevZoom = null;
  let prevPan = null;
  function panZoom(element) {
    let state = svgPanZoom(element, {
      minZoom: 0.1,
      maxZoom: 50,
      zoomScaleSensitivity: 0.5,
    });
    if (prevZoom) {
      state.zoom(prevZoom);
      state.pan(prevPan);
    }
    return {
      update() {},
      destroy() {
        prevZoom = state.getZoom();
        prevPan = state.getPan();
      },
    };
  }

  // We want this behavior in all modes, so keep it here
  function setFocus(f) {
    $clickedFeature = f;
    if ($clickedFeature == null) {
      // TODO Click and drag incorrectly triggers this
      mode.set({ mode: "neutral" });
    } else if ($clickedFeature.geometry.type == "LineString") {
      mode.set({ mode: "find-width", road: $clickedFeature });
    } else if ($clickedFeature.geometry.type == "Point") {
      mode.set({
        mode: "intersection-geometry",
        intersection: $clickedFeature,
      });
    } else if ($clickedFeature.geometry.type == "Polygon") {
      mode.set({ mode: "neutral" });
    }
  }

  let mapDiv;
  $: if (mapDiv && $mapContents) {
    mapDiv.innerHTML = "";
    mapDiv.appendChild($mapContents);
  }
</script>

{#key $mode}
  <svg use:panZoom on:click={() => setFocus(null)}>
    {#each roads as f}
      {#if $showRealRoadWidth && f.properties.polygon}
        <polygon
          points={polygonToSvg(JSON.parse(f.properties.polygon))}
          on:click={() => setFocus(f)}
          class="road-outline"
          class:clicked={$clickedFeature == f}
        />
      {:else}
        <polyline
          points={gjToSvg(f.geometry.coordinates)}
          on:click={() => setFocus(f)}
          class:clicked={$clickedFeature == f}
        />
      {/if}
    {/each}
    {#each intersections as f}
      <circle
        cx={f.geometry.coordinates[0]}
        cy={f.geometry.coordinates[1]}
        r="1"
        on:click={() => setFocus(f)}
        class:clicked={$clickedFeature == f}
      />
    {/each}
    {#each buildings as f}
      <polygon
        points={gjToSvg(f.geometry.coordinates[0])}
        on:click={() => setFocus(f)}
        class:clicked={$clickedFeature == f}
      />
    {/each}
    <g bind:this={mapDiv} />
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

  .road-outline {
    fill: none;
  }

  circle {
    fill: red;
  }
  circle:hover {
    fill: blue;
  }

  .clicked {
    stroke: yellow;
    stroke-width: 1;
  }
</style>
