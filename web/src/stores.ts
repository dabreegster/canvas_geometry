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

export let mode: Writable<Mode> = writable({ mode: "neutral" });
