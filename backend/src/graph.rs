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

impl Edge {
    fn other_node(&self, n: NodeID) -> NodeID {
        // TODO Loops
        if self.node1 == n {
            self.node2
        } else {
            assert_eq!(self.node2, n);
            self.node1
        }
    }
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

    pub fn trace_graph_loop(&mut self, node: usize) {
        // Find a loop on this node
        let Some(nodes) = self.find_cycle(NodeID(node)) else {
            info!("no loop");
            return;
        };
        info!("Got path {:?}", nodes);
        // For all the nodes in there, calculate the centroid
        // Remove all the edges in the path
        // Remove all the nodes in the path. Create a new one with the centroid and preserving all
        // the old edges.
    }

    fn find_cycle(&self, on_node: NodeID) -> Option<Vec<NodeID>> {
        // Offline, this is an awful approach, but I want to move on
        let mut queue: Vec<Vec<NodeID>> = Vec::new();
        queue.push(vec![on_node]);

        while let Some(current_path) = queue.pop() {
            let last_node = *current_path.last().unwrap();
            if current_path.len() > 2 && last_node == on_node {
                return Some(current_path);
            }

            // Limit length
            if current_path.len() > 6 {
                continue;
            }

            for edge in &self.nodes[&last_node].edges {
                let next_node = self.edges[edge].other_node(last_node);
                // Don't double-back
                if current_path.contains(&next_node) {
                    // Unless we found the loop
                    if next_node == on_node && current_path.len() > 2 {
                        // Let it go
                    } else {
                        continue;
                    }
                }
                let mut new_path = current_path.clone();
                new_path.push(next_node);
                queue.push(new_path);
            }
        }

        None
    }
}
