use std::collections::HashMap;

use anyhow::Result;
use geo::{Coord, Geometry, GeometryCollection, LineString, MapCoordsInPlace, Polygon};

use crate::mercator::Mercator;
use crate::osm::OsmID;
use crate::parse_osm::Element;
use crate::{Building, Diagram};

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
                    buildings.push(Building {
                        id: OsmID::Way(id),
                        polygon,
                    });
                }
            }
            Element::Relation { .. } => {}
        }
    }

    // TODO expensive
    let collection: GeometryCollection = buildings
        .iter()
        .map(|b| Geometry::Polygon(b.polygon.clone()))
        .collect::<Vec<_>>()
        .into();
    let mercator = Mercator::from(collection).unwrap();
    for b in &mut buildings {
        b.polygon.map_coords_in_place(|pt| mercator.to_mercator(pt));
    }

    Ok(Diagram { buildings })
}
