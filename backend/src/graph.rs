use std::collections::{HashMap, HashSet};

use geo::{LineString, Point};
use serde::Serialize;

use crate::{Intersection, IntersectionID, Road, RoadID};

/// Much more mutable than a MapModel, but refers back to original roads and intersections.
#[derive(Serialize)]
pub struct Graph {
    edges: HashMap<EdgeID, Edge>,
    nodes: HashMap<NodeID, Node>,
    next_edge_id: usize,
    next_node_id: usize,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize)]
pub struct EdgeID(pub usize);
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize)]
pub struct NodeID(pub usize);

#[derive(Serialize)]
struct Edge {
    id: EdgeID,
    node1: NodeID,
    node2: NodeID,

    linestring: LineString,
    roads: HashSet<RoadID>,
}

#[derive(Serialize)]
struct Node {
    id: NodeID,
    edges: HashSet<EdgeID>,

    point: Point,
    intersections: HashSet<IntersectionID>,
}

impl Graph {
    pub fn new_from_map(roads: &Vec<Road>, intersections: &Vec<Intersection>) -> Self {
        let mut graph = Graph {
            edges: HashMap::new(),
            nodes: HashMap::new(),
            next_edge_id: 0,
            next_node_id: 0,
        };

        // Everything starts 1:1 with the map
        let mut i_to_node = HashMap::new();
        for i in intersections {
            let id = graph.new_node_id();
            graph.nodes.insert(
                id,
                Node {
                    id,
                    edges: HashSet::new(),

                    point: i.point,
                    intersections: HashSet::from([i.id]),
                },
            );
            i_to_node.insert(i.id, id);
        }

        for r in roads {
            let id = graph.new_edge_id();
            let node1 = i_to_node[&r.src_i];
            let node2 = i_to_node[&r.dst_i];
            graph.edges.insert(
                id,
                Edge {
                    id,
                    node1,
                    node2,

                    linestring: r.linestring.clone(),
                    roads: HashSet::from([r.id]),
                },
            );
            graph.nodes.get_mut(&node1).unwrap().edges.insert(id);
            graph.nodes.get_mut(&node2).unwrap().edges.insert(id);
        }

        graph
    }

    fn new_edge_id(&mut self) -> EdgeID {
        let x = EdgeID(self.next_edge_id);
        self.next_edge_id += 1;
        x
    }

    fn new_node_id(&mut self) -> NodeID {
        let x = NodeID(self.next_node_id);
        self.next_node_id += 1;
        x
    }

    pub fn render(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
