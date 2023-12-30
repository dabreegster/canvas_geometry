use std::collections::HashMap;

use anyhow::Result;
use geo::{Coord, Geometry, GeometryCollection, LineString, Point, Polygon};
use osm_reader::{Element, NodeID, OsmID, WayID};

use crate::graph::Graph;
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
        mercator.to_mercator_in_place(&mut r.linestring);
    }
    for i in &mut intersections {
        mercator.to_mercator_in_place(&mut i.point);
    }
    for b in &mut buildings {
        mercator.to_mercator_in_place(&mut b.polygon);
    }

    let graph = Graph::new_from_map(&roads, &intersections);
    let mut map = MapModel {
        mercator,
        roads,
        intersections,
        buildings,
        graph,
    };
    crate::find_road_width::find_all(&mut map);
    Ok(map)
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
    let mut node_to_intersection = HashMap::new();
    let mut intersections = Vec::new();
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
                    let i = if let Some(i) = node_to_intersection.get(&n) {
                        *i
                    } else {
                        let i = IntersectionID(intersections.len());
                        node_to_intersection.insert(n, i);
                        intersections.push(Intersection {
                            id: i,
                            node: n,
                            point: Point(point),
                            roads: Vec::new(),
                        });
                        i
                    };
                    intersections[i.0].roads.push(road_id);
                }

                roads.push(Road {
                    id: road_id,
                    way: way.id,
                    node1,
                    node2: node,
                    src_i: node_to_intersection[&node1],
                    dst_i: node_to_intersection[&node],
                    linestring: LineString::new(std::mem::take(&mut pts)),
                    tags: way.tags.clone(),

                    max_left_width: None,
                    max_right_width: None,
                    polygon: None,
                });

                // Start the next edge
                node1 = node;
                pts.push(node_mapping[&node]);
            }
        }
    }

    (roads, intersections)
}
