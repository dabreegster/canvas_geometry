<script lang="ts">
  import init, { Diagram } from "backend";
  import { onMount } from "svelte";
  import Layout from "./Layout.svelte";
  import Loading from "./Loading.svelte";

  onMount(async () => {
    await init();
  });

  let diagram: Diagram | undefined = undefined;
  let loading = false;

  let fileInput: HTMLInputElement;
  async function loadFile(e: Event) {
    try {
      loading = true;
      let buffer = await fileInput.files![0].arrayBuffer();
      diagram = new Diagram(new Uint8Array(buffer));
      loading = false;
    } catch (err) {
      window.alert(`Couldn't open this file: ${err}`);
    }
  }
</script>

<Layout>
  <div slot="left">
    <label>
      <input bind:this={fileInput} on:change={loadFile} type="file" />
    </label>
  </div>
  <div slot="main" style="position:relative; width: 100%; height: 100vh;">
    {#if diagram}
      <pre>{diagram.render()}</pre>
    {/if}
  </div>
</Layout>
<Loading {loading} />
