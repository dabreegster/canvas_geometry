<script lang="ts">
  import svgPanZoom from "svg-pan-zoom";

  export let gj;

  let roads = gj.features.filter((f) => f.geometry.type == "LineString");
  let buildings = gj.features.filter((f) => f.geometry.type == "Polygon");

  function gjToSvg(points) {
    return points.map((pt) => `${pt[0]},${pt[1]}`).join(" ");
  }

  function panZoom(element) {
    svgPanZoom(element, {
      minZoom: 0.1,
      maxZoom: 50,
      zoomScaleSensitivity: 0.5,
    });
  }
</script>

<svg use:panZoom>
  {#each roads as f}
    <polyline points={gjToSvg(f.geometry.coordinates)} />
  {/each}
  {#each buildings as f}
    <polygon points={gjToSvg(f.geometry.coordinates[0])} />
  {/each}
</svg>

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
</style>
