<script lang="ts">
  import { mode } from "./stores";

  export let clickedFeature;
</script>

{#if clickedFeature}
  {#if $mode.mode == "neutral"}
    {#if clickedFeature.geometry.type == "LineString"}
      <div>
        <button
          on:click={() =>
            mode.set({ mode: "find-width", road: clickedFeature })}
          >Find width</button
        >
      </div>
    {:else if clickedFeature.geometry.type == "Point"}
      <div>
        <button
          on:click={() =>
            mode.set({
              mode: "intersection-geometry",
              intersection: clickedFeature,
            })}>Intersection geometry</button
        >
      </div>
    {/if}

    <table>
      <tbody>
        {#each Object.entries(clickedFeature.properties) as [key, value]}
          <tr><td>{key}</td><td>{value}</td></tr>
        {/each}
      </tbody>
    </table>
  {:else if $mode.mode == "find-width"}
    <p>Finding width of this road...</p>
    <div>
      <button on:click={() => mode.set({ mode: "neutral" })}>Back</button>
    </div>
  {:else if $mode.mode == "intersection-geometry"}
    <p>Intersection geometry</p>
    <div>
      <button on:click={() => mode.set({ mode: "neutral" })}>Back</button>
    </div>
  {/if}
{/if}

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
