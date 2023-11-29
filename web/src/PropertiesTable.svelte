<script lang="ts">
  export let tags: { [name: string]: any };
</script>

<table>
  <tbody>
    {#each Object.entries(tags) as [key, value]}
      <tr><td>{key}</td><td>{value}</td></tr>
    {/each}
  </tbody>
</table>

<style>
  td {
    border: solid;
  }
</style>
