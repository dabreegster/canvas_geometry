use std::collections::HashMap;

use anyhow::Result;
use geo::{Coord, LineString, Polygon};

use crate::osm::OsmID;
use crate::parse_osm::Element;
use crate::Diagram;

pub fn scrape_osm(input_bytes: &[u8]) -> Result<Diagram> {
    let mut node_mapping = HashMap::new();
    let mut buildings = Vec::new();
    for elem in crate::parse_osm::parse_osm(input_bytes)? {
        match elem {
            Element::Node { id, pt, .. } => {
                node_mapping.insert(id, pt);
            }
            Element::Way { id, node_ids, tags } => {
                if tags.contains_key("building") {
                    // geo closes the polygon for us
                    let polygon = Polygon::new(
                        LineString::new(
                            node_ids
                                .into_iter()
                                .map(|id| Coord::from(node_mapping[&id]))
                                .collect(),
                        ),
                        Vec::new(),
                    );
                    buildings.push((OsmID::Way(id), polygon));
                }
            }
            Element::Relation { .. } => {}
        }
    }
    Ok(Diagram { buildings })
}
