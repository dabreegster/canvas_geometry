#[macro_use]
extern crate log;

use std::sync::Once;

use geo::{point, polygon, Geometry, GeometryCollection};
use geojson::FeatureCollection;
use std::iter::FromIterator;
use wasm_bindgen::prelude::*;

static START: Once = Once::new();

#[wasm_bindgen]
pub struct Diagram {}

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

        info!("Got {} bytes", input_bytes.len());

        Ok(Diagram {})
    }

    /// Returns a GeoJSON string
    #[wasm_bindgen()]
    pub fn render(&mut self) -> Result<String, JsValue> {
        // TODO Dummy output
        let poly: Geometry<f64> = polygon![
            (x: -111., y: 45.),
            (x: -111., y: 41.),
            (x: -104., y: 41.),
            (x: -104., y: 45.),
        ]
        .into();

        let point: Geometry<f64> = point!(x: 1.0, y: 2.0).into();

        let geometry_collection = GeometryCollection::from_iter(vec![poly, point]);
        let feature_collection = FeatureCollection::from(&geometry_collection);

        let out = serde_json::to_string(&feature_collection).map_err(err_to_js)?;
        Ok(out)
    }
}

fn err_to_js<E: std::fmt::Display>(err: E) -> JsValue {
    JsValue::from_str(&err.to_string())
}
