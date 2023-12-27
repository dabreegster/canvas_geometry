<script lang="ts">
  import init, { MapModel } from "backend";
  import { onMount } from "svelte";
  import xmlUrl from "../assets/input.osm?url";
  import Canvas from "./Canvas.svelte";
  import FindRoadWidth from "./FindRoadWidth.svelte";
  import IntersectionGeometry from "./IntersectionGeometry.svelte";
  import Layout from "./Layout.svelte";
  import Loading from "./Loading.svelte";
  import Neutral from "./Neutral.svelte";
  import {
    map,
    mapContents,
    mode,
    showRealRoadWidth,
    sidebarContents,
  } from "./stores";

  onMount(async () => {
    await init();
    try {
      loading = true;
      let resp = await fetch(xmlUrl);
      let buffer = await resp.arrayBuffer();
      $map = new MapModel(new Uint8Array(buffer));
    } catch (err) {
      window.alert(`Couldn't open from URL ${xmlUrl}: ${err}`);
    }
    loading = false;
  });

  let loading = false;

  let fileInput: HTMLInputElement;
  async function loadFile(e: Event) {
    try {
      loading = true;
      let buffer = await fileInput.files![0].arrayBuffer();
      $map = new MapModel(new Uint8Array(buffer));
    } catch (err) {
      window.alert(`Couldn't open this file: ${err}`);
    }
    loading = false;
  }

  let sidebarDiv;
  $: if (sidebarDiv && $sidebarContents) {
    sidebarDiv.innerHTML = "";
    sidebarDiv.appendChild($sidebarContents);
  }
</script>

<Layout>
  <div slot="left">
    <label>
      <input bind:this={fileInput} on:change={loadFile} type="file" />
    </label>
    <div>
      <label>
        <input type="checkbox" bind:checked={$showRealRoadWidth} /> Show calculated
        road widths
      </label>
    </div>
    <div bind:this={sidebarDiv} />
  </div>
  <div slot="main" style="position:relative; width: 100%; height: 100vh;">
    {#if $map}
      <Canvas gj={JSON.parse($map.render())} />
    {/if}
  </div>
</Layout>
<Loading {loading} />
{#if $mode.mode == "neutral"}
  <Neutral />
{:else if $mode.mode == "find-width"}
  <FindRoadWidth />
{:else if $mode.mode == "intersection-geometry"}
  <IntersectionGeometry />
{/if}
