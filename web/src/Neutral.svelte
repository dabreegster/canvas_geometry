<script lang="ts">
  import SplitComponent from "./SplitComponent.svelte";
  import { clickedFeature, mode } from "./stores";
</script>

<SplitComponent>
  <div slot="sidebar">
    {#if $clickedFeature}
      {#if $clickedFeature.geometry.type == "LineString"}
        <div>
          <button
            on:click={() =>
              mode.set({ mode: "find-width", road: $clickedFeature })}
            >Find width</button
          >
        </div>
      {:else if $clickedFeature.geometry.type == "Point"}
        <div>
          <button
            on:click={() =>
              mode.set({
                mode: "intersection-geometry",
                intersection: $clickedFeature,
              })}>Intersection geometry</button
          >
        </div>
      {/if}

      <table>
        <tbody>
          {#each Object.entries($clickedFeature.properties) as [key, value]}
            <tr><td>{key}</td><td>{value}</td></tr>
          {/each}
        </tbody>
      </table>
    {/if}
  </div>
</SplitComponent>

<style>
  button {
    font-size: 30px;
    margin: 20px;
  }

  td {
    border: solid;
    padding: 8px;
  }
</style>
