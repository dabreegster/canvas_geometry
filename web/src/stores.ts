import { MapModel } from "backend";
import { writable, type Writable } from "svelte/store";

export type Mode =
  | {
      mode: "neutral";
    }
  | {
      mode: "find-width";
      // TODO gj feature type
      road: any;
    }
  | {
      mode: "intersection-polygon";
      intersection: any;
    };

export let map: Writable<MapModel | null> = writable(null);
export let mode: Writable<Mode> = writable({ mode: "neutral" });
export let clickedFeature: Writable<Feature | null> = writable(null);

export let sidebarContents = writable(null);
export let mapContents = writable(null);
