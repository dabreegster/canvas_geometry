#[macro_use]
extern crate log;

use std::collections::HashMap;
use std::sync::Once;

use geo::{LineString, Point, Polygon};
use geojson::GeoJson;
use serde::Serialize;
use wasm_bindgen::prelude::*;

mod find_road_width;
mod graph;
mod intersection_geometry;
mod math;
mod mercator;
mod output;
mod scrape;

static START: Once = Once::new();

#[wasm_bindgen]
pub struct MapModel {
    #[allow(unused)]
    mercator: mercator::Mercator,
    roads: Vec<Road>,
    intersections: Vec<Intersection>,
    buildings: Vec<Building>,

    // TODO Weird to embed like this, but easier to prototype
    graph: graph::Graph,
    graph_undo_stack: Vec<graph::Graph>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize)]
pub struct RoadID(pub usize);
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize)]
pub struct IntersectionID(pub usize);

pub struct Road {
    id: RoadID,
    way: osm_reader::WayID,
    node1: osm_reader::NodeID,
    node2: osm_reader::NodeID,
    src_i: IntersectionID,
    dst_i: IntersectionID,
    linestring: LineString,
    tags: HashMap<String, String>,

    // Derived a bit later
    max_left_width: Option<f64>,
    max_right_width: Option<f64>,
    polygon: Option<Polygon>,
}

pub struct Intersection {
    id: IntersectionID,
    node: osm_reader::NodeID,
    point: Point,
    roads: Vec<RoadID>,
}

struct Building {
    id: osm_reader::OsmID,
    polygon: Polygon,
    tags: HashMap<String, String>,
}

#[wasm_bindgen]
impl MapModel {
    /// Call with bytes of an osm.pbf or osm.xml string
    #[wasm_bindgen(constructor)]
    pub fn new(input_bytes: &[u8]) -> Result<MapModel, JsValue> {
        // Panics shouldn't happen, but if they do, console.log them.
        console_error_panic_hook::set_once();
        START.call_once(|| {
            console_log::init_with_level(log::Level::Info).unwrap();
        });

        scrape::scrape_osm(input_bytes).map_err(err_to_js)
    }

    /// Returns a GeoJSON string
    #[wasm_bindgen()]
    pub fn render(&mut self) -> Result<String, JsValue> {
        let mut features = Vec::new();

        for r in &self.roads {
            features.push(r.to_geojson());
        }
        for i in &self.intersections {
            features.push(i.to_geojson());
        }
        for b in &self.buildings {
            features.push(b.to_geojson());
        }

        let gj = GeoJson::from(features);
        let out = serde_json::to_string(&gj).map_err(err_to_js)?;
        Ok(out)
    }

    #[wasm_bindgen(js_name = findRoadWidth)]
    pub fn find_road_width(&self, r: usize) -> Result<String, JsValue> {
        let obj = find_road_width::find_road_width(self, RoadID(r));
        let out = serde_json::to_string(&obj).map_err(err_to_js)?;
        Ok(out)
    }

    #[wasm_bindgen(js_name = findIntersectionGeometry)]
    pub fn find_intersection_geometry(&self, i: usize) -> Result<String, JsValue> {
        let obj = intersection_geometry::find_intersection_geometry(self, IntersectionID(i));
        let out = serde_json::to_string(&obj).map_err(err_to_js)?;
        Ok(out)
    }

    // Graph stuff
    #[wasm_bindgen(js_name = renderGraph)]
    pub fn render_graph(&self) -> String {
        self.graph.render()
    }

    #[wasm_bindgen(js_name = traceGraphLoop)]
    pub fn trace_graph_loop(&mut self, node: usize) {
        self.graph_undo_stack.push(self.graph.clone());
        self.graph.trace_graph_loop(node)
    }

    #[wasm_bindgen(js_name = undoGraph)]
    pub fn undo_graph(&mut self) {
        if let Some(g) = self.graph_undo_stack.pop() {
            self.graph = g;
        }
    }
}

fn err_to_js<E: std::fmt::Display>(err: E) -> JsValue {
    JsValue::from_str(&err.to_string())
}
