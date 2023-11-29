<script lang="ts">
  import svgPanZoom from "svg-pan-zoom";

  export let gj;

  function gjPolygonToSvg(f) {
    let points = "";
    for (let pt of f.geometry.coordinates[0]) {
      points += `${pt[0]},${pt[1]} `;
    }
    return points;
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
  {#each gj.features as f}
    <polygon points={gjPolygonToSvg(f)} />
  {/each}
</svg>

<style>
  svg {
    width: 100%;
    height: 100%;
    background-color: grey;
  }

  polygon {
    fill: red;
    stroke: black;
  }

  polygon:hover {
    fill: blue;
  }
</style>
