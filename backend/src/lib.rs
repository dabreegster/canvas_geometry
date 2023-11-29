#[macro_use]
extern crate log;

use std::sync::Once;

use geo::Polygon;
use geojson::{Feature, GeoJson, Geometry};
use wasm_bindgen::prelude::*;

mod osm;
mod parse_osm;
mod scrape;

static START: Once = Once::new();

#[wasm_bindgen]
pub struct Diagram {
    buildings: Vec<(osm::OsmID, Polygon)>,
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

        for (id, polygon) in &self.buildings {
            let mut f = Feature::from(Geometry::from(polygon));
            f.set_property("id", id.to_string());
            features.push(f);
        }

        let gj = GeoJson::from(features);
        let out = serde_json::to_string(&gj).map_err(err_to_js)?;
        Ok(out)
    }
}

fn err_to_js<E: std::fmt::Display>(err: E) -> JsValue {
    JsValue::from_str(&err.to_string())
}
