#[macro_use]
extern crate log;

use std::collections::HashMap;
use std::sync::Once;

use geo::{LineString, Polygon};
use geojson::GeoJson;
use wasm_bindgen::prelude::*;

mod mercator;
mod osm;
mod output;
mod parse_osm;
mod scrape;

static START: Once = Once::new();

#[wasm_bindgen]
pub struct Diagram {
    roads: Vec<Road>,
    buildings: Vec<Building>,
}

struct Road {
    way: osm::WayID,
    node1: osm::NodeID,
    node2: osm::NodeID,
    linestring: LineString,
    tags: HashMap<String, String>,
}

struct Building {
    id: osm::OsmID,
    polygon: Polygon,
    tags: HashMap<String, String>,
}

#[wasm_bindgen]
impl Diagram {
    /// Call with bytes of an osm.pbf or osm.xml string
    #[wasm_bindgen(constructor)]
    pub fn new(input_bytes: &[u8]) -> Result<Diagram, JsValue> {
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
        for b in &self.buildings {
            features.push(b.to_geojson());
        }

        let gj = GeoJson::from(features);
        let out = serde_json::to_string(&gj).map_err(err_to_js)?;
        Ok(out)
    }
}

fn err_to_js<E: std::fmt::Display>(err: E) -> JsValue {
    JsValue::from_str(&err.to_string())
}
