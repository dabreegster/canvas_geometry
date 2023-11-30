# canvas_geometry

Playing around with OSM, georust, and canvas

## Thoughts

- Biggest annoyance is the loss of type-safety
  - There's no need to output as GJ, actually -- just serde as JSON
  - But can we generate TS types on the other end? Even if we lose things like RoadID wrappers?
