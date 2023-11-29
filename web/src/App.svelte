<script lang="ts">
  import init, { Diagram } from "backend";
  import { onMount } from "svelte";
  import xmlUrl from "../assets/input.osm?url";
  import Canvas from "./Canvas.svelte";
  import Layout from "./Layout.svelte";
  import Loading from "./Loading.svelte";
  import Sidebar from "./Sidebar.svelte";

  onMount(async () => {
    await init();
    try {
      loading = true;
      let resp = await fetch(xmlUrl);
      let buffer = await resp.arrayBuffer();
      diagram = new Diagram(new Uint8Array(buffer));
    } catch (err) {
      window.alert(`Couldn't open from URL ${xmlUrl}: ${err}`);
    }
    loading = false;
  });

  let diagram: Diagram | undefined = undefined;
  let loading = false;
  let clickedFeature = null;

  let fileInput: HTMLInputElement;
  async function loadFile(e: Event) {
    try {
      loading = true;
      let buffer = await fileInput.files![0].arrayBuffer();
      diagram = new Diagram(new Uint8Array(buffer));
    } catch (err) {
      window.alert(`Couldn't open this file: ${err}`);
    }
    loading = false;
  }
</script>

<Layout>
  <div slot="left">
    <label>
      <input bind:this={fileInput} on:change={loadFile} type="file" />
    </label>
    <Sidebar {clickedFeature} />
  </div>
  <div slot="main" style="position:relative; width: 100%; height: 100vh;">
    {#if diagram}
      <Canvas gj={JSON.parse(diagram.render())} bind:clickedFeature />
    {/if}
  </div>
</Layout>
<Loading {loading} />
