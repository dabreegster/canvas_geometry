use std::collections::HashMap;

use anyhow::Result;
use geo::{Coord, Geometry, GeometryCollection, LineString, MapCoordsInPlace, Polygon};

use crate::mercator::Mercator;
use crate::osm::{NodeID, OsmID, WayID};
use crate::parse_osm::Element;
use crate::{Building, Diagram, Road};

struct Way {
    id: WayID,
    node_ids: Vec<NodeID>,
}

pub fn scrape_osm(input_bytes: &[u8]) -> Result<Diagram> {
    let mut node_mapping = HashMap::new();
    let mut highways = Vec::new();
    let mut buildings = Vec::new();
    for elem in crate::parse_osm::parse_osm(input_bytes)? {
        match elem {
            Element::Node { id, pt, .. } => {
                node_mapping.insert(id, pt);
            }
            Element::Way { id, node_ids, tags } => {
                if tags.contains_key("highway") {
                    highways.push(Way { id, node_ids });
                } else if tags.contains_key("building") {
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

    let mut roads = split_edges(&node_mapping, highways);

    // TODO expensive
    let collection: GeometryCollection = buildings
        .iter()
        .map(|b| Geometry::Polygon(b.polygon.clone()))
        .chain(
            roads
                .iter()
                .map(|r| Geometry::LineString(r.linestring.clone())),
        )
        .collect::<Vec<_>>()
        .into();
    let mercator = Mercator::from(collection).unwrap();
    for r in &mut roads {
        r.linestring
            .map_coords_in_place(|pt| mercator.to_mercator(pt));
    }
    for b in &mut buildings {
        b.polygon.map_coords_in_place(|pt| mercator.to_mercator(pt));
    }

    Ok(Diagram { roads, buildings })
}

fn split_edges(node_mapping: &HashMap<NodeID, Coord>, ways: Vec<Way>) -> Vec<Road> {
    // Count how many ways reference each node
    let mut node_counter: HashMap<NodeID, usize> = HashMap::new();
    for way in &ways {
        for node in &way.node_ids {
            *node_counter.entry(*node).or_insert(0) += 1;
        }
    }

    // Split each way into edges
    let mut intersections = HashMap::new();
    let mut roads = Vec::new();
    for way in ways {
        let mut node1 = way.node_ids[0];
        let mut pts = Vec::new();

        let num_nodes = way.node_ids.len();
        for (idx, node) in way.node_ids.into_iter().enumerate() {
            pts.push(node_mapping[&node]);
            // Edges start/end at intersections between two ways. The endpoints of the way also
            // count as intersections.
            let is_endpoint =
                idx == 0 || idx == num_nodes - 1 || *node_counter.get(&node).unwrap() > 1;
            if is_endpoint && pts.len() > 1 {
                intersections.insert(node1, pts[0]);
                intersections.insert(node, *pts.last().unwrap());
                roads.push(Road {
                    way: way.id,
                    node1,
                    node2: node,
                    linestring: LineString::new(std::mem::take(&mut pts)),
                });

                // Start the next edge
                node1 = node;
                pts.push(node_mapping[&node]);
            }
        }
    }

    // Ignore intersections for now
    roads
}
