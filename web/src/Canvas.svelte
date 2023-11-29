<script lang="ts">
  export let gj;

  function gjPolygonToSvg(f) {
    let points = "";
    for (let pt of f.geometry.coordinates[0]) {
      points += `${pt[0]},${pt[1]} `;
    }
    return points;
  }
</script>

<svg>
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
