use std::collections::HashMap;

use anyhow::Result;
use geo::{Coord, LineString, Polygon};
use osm_reader::{Element, OsmID};
use utils::Tags;

use crate::graph::Graph;
use crate::{Building, Intersection, IntersectionID, MapModel, Road, RoadID};

pub fn scrape_osm(input_bytes: &[u8]) -> Result<MapModel> {
    let mut node_mapping = HashMap::new();
    let mut highways = Vec::new();
    let mut buildings = Vec::new();
    osm_reader::parse(input_bytes, |elem| match elem {
        Element::Node { id, lon, lat, .. } => {
            node_mapping.insert(id, Coord { x: lon, y: lat });
        }
        Element::Way {
            id, node_ids, tags, ..
        } => {
            let tags: Tags = tags.into();
            if tags.has("highway") {
                highways.push(utils::osm2graph::Way { id, node_ids, tags });
            } else if tags.has("building") {
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
        Element::Bounds { .. } => {}
    })?;

    let osm_graph = utils::osm2graph::Graph::from_scraped_osm(node_mapping, highways);
    // Copy all the fields
    let intersections: Vec<Intersection> = osm_graph
        .intersections
        .into_iter()
        .map(|i| Intersection {
            id: IntersectionID(i.id.0),
            point: i.point,
            node: i.osm_node,
            roads: i.edges.into_iter().map(|e| RoadID(e.0)).collect(),
        })
        .collect();

    // Add in a bit
    let roads: Vec<Road> = osm_graph
        .edges
        .into_iter()
        .map(|e| Road {
            id: RoadID(e.id.0),
            src_i: IntersectionID(e.src.0),
            dst_i: IntersectionID(e.dst.0),
            way: e.osm_way,
            node1: e.osm_node1,
            node2: e.osm_node2,
            linestring: e.linestring,
            tags: e.osm_tags,
            max_left_width: None,
            max_right_width: None,
            polygon: None,
        })
        .collect();

    for b in &mut buildings {
        osm_graph.mercator.to_mercator_in_place(&mut b.polygon);
    }

    let graph = Graph::new_from_map(&roads, &intersections);
    let mut map = MapModel {
        mercator: osm_graph.mercator,
        roads,
        intersections,
        buildings,
        graph,
        graph_undo_stack: Vec::new(),
    };
    crate::find_road_width::find_all(&mut map);
    Ok(map)
}
