<script lang="ts">
  import init, { MapModel } from "backend";
  import { onMount } from "svelte";
  import Loading from "./Loading.svelte";
  import { map } from "./stores";

  let example = "st_georges_cycletrack";
  let useLocalVite = false;
  let loading = false;

  onMount(async () => {
    await init();

    // When running locally if a vite public/ directory is set up, load from that for speed
    try {
      let resp = await fetch(`/${example}/input.osm`, {
        method: "HEAD",
      });

      // For quicker dev
      loadExample();
    } catch (err) {}
  });

  async function loadExample() {
    // TODO Depending on useLocalVite
    let url = `/${example}/input.osm`;
    try {
      loading = true;
      let resp = await fetch(url);
      let buffer = await resp.arrayBuffer();
      $map = new MapModel(new Uint8Array(buffer));
      console.log(`Loaded ${example}`);
    } catch (err) {
      window.alert(`Couldn't open from URL ${url}: ${err}`);
    }
    loading = false;
  }

  let fileInput: HTMLInputElement;
  async function loadFile(e: Event) {
    try {
      loading = true;
      let buffer = await fileInput.files![0].arrayBuffer();
      $map = new MapModel(new Uint8Array(buffer));
      example = "";
    } catch (err) {
      window.alert(`Couldn't open this file: ${err}`);
    }
    loading = false;
  }

  let examples = [
    "arizona_highways",
    "aurora_sausage_link",
    "borough_sausage_links",
    "bristol_contraflow_cycleway",
    "bristol_sausage_links",
    "cycleway_rejoin_road",
    "degenerate_bug",
    "frederiksted",
    "fremantle_placement",
    "i5_exit_ramp",
    "kingsway_junction",
    "leeds_cycleway",
    "montlake_roundabout",
    "northgate_dual_carriageway",
    "oneway_loop",
    "overlapping_service_roads",
    "perth_peanut_roundabout",
    "perth_stretched_lights",
    "quad_intersection",
    "roosevelt_cycletrack",
    "seattle_slip_lane",
    "seattle_triangle",
    "service_road_loop",
    "st_georges_cycletrack",
    "taipei",
    "tempe_light_rail",
    "tempe_split",
    "tiny_loop",
    "tiny_roundabout",
  ];
</script>

<Loading {loading} />

<div style="border: 1px solid black; padding: 8px;">
  <div>
    <label>
      Load an osm.xml or a .pbf file:
      <input bind:this={fileInput} on:change={loadFile} type="file" />
    </label>
  </div>

  <div>
    <label>
      Or load an example:
      <select bind:value={example} on:change={loadExample}>
        <option value="">Custom file loaded</option>
        {#each examples as example}
          <option value={example}>{example}</option>
        {/each}
      </select>
    </label>
  </div>
</div>
