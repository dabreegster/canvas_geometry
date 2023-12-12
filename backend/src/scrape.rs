use std::collections::{BTreeMap, HashMap};

use anyhow::Result;
use geo::{Coord, Geometry, GeometryCollection, LineString, MapCoordsInPlace, Point, Polygon};
use osm_reader::{Element, NodeID, OsmID, WayID};

use crate::mercator::Mercator;
use crate::{Building, Intersection, IntersectionID, MapModel, Road, RoadID};

struct Way {
    id: WayID,
    node_ids: Vec<NodeID>,
    tags: HashMap<String, String>,
}

pub fn scrape_osm(input_bytes: &[u8]) -> Result<MapModel> {
    let mut node_mapping = HashMap::new();
    let mut highways = Vec::new();
    let mut buildings = Vec::new();
    osm_reader::parse(input_bytes, |elem| match elem {
        Element::Node { id, lon, lat, .. } => {
            node_mapping.insert(id, Coord { x: lon, y: lat });
        }
        Element::Way { id, node_ids, tags } => {
            if tags.contains_key("highway") {
                highways.push(Way { id, node_ids, tags });
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
                    tags,
                });
            }
        }
        Element::Relation { .. } => {}
    })?;

    let (mut roads, mut intersections) = split_edges(&node_mapping, highways);

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
    for i in &mut intersections {
        i.point.map_coords_in_place(|pt| mercator.to_mercator(pt));
    }
    for b in &mut buildings {
        b.polygon.map_coords_in_place(|pt| mercator.to_mercator(pt));
    }

    Ok(MapModel {
        mercator,
        roads,
        intersections,
        buildings,
    })
}

fn split_edges(
    node_mapping: &HashMap<NodeID, Coord>,
    ways: Vec<Way>,
) -> (Vec<Road>, Vec<Intersection>) {
    // Count how many ways reference each node
    let mut node_counter: HashMap<NodeID, usize> = HashMap::new();
    for way in &ways {
        for node in &way.node_ids {
            *node_counter.entry(*node).or_insert(0) += 1;
        }
    }

    // Split each way into edges
    let mut intersections = BTreeMap::new();
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
                let road_id = RoadID(roads.len());

                for (n, point) in [(node1, pts[0]), (node, *pts.last().unwrap())] {
                    let next_id = IntersectionID(intersections.len());
                    intersections
                        .entry(n)
                        .or_insert_with(|| Intersection {
                            id: next_id,
                            node: n,
                            point: Point(point),
                            roads: Vec::new(),
                        })
                        .roads
                        .push(road_id);
                }

                roads.push(Road {
                    id: road_id,
                    way: way.id,
                    node1,
                    node2: node,
                    linestring: LineString::new(std::mem::take(&mut pts)),
                    tags: way.tags.clone(),
                });

                // Start the next edge
                node1 = node;
                pts.push(node_mapping[&node]);
            }
        }
    }

    let intersections = intersections.into_values().collect();

    (roads, intersections)
}
