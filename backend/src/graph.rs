use std::collections::{HashMap, HashSet, BinaryHeap};

use geo::{LineString, Point, EuclideanLength};
use serde::Serialize;

use crate::{Intersection, IntersectionID, Road, RoadID};
use crate::priority_queue::PriorityQueueItem;

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
        let Some(nodes) = self.pathfind(NodeID(node), NodeID(node)) else {
            info!("no loop");
            return;
        };
        info!("Got path {:?}", nodes);
        // For all the nodes in there, calculate the centroid
        // Remove all the edges in the path
        // Remove all the nodes in the path. Create a new one with the centroid and preserving all
        // the old edges.
    }

    fn pathfind(&self, node1: NodeID, node2: NodeID) -> Option<Vec<NodeID>> {
        // Offline without petgraph...
        let mut queue: BinaryHeap<PriorityQueueItem<usize, NodeID>> = BinaryHeap::new();
        queue.push(PriorityQueueItem {
            cost: 0,
            value: node1,
        });

        let mut visited: HashSet<NodeID> = HashSet::new();
        let mut backref: HashMap<NodeID, NodeID> = HashMap::new();
        while let Some(current) = queue.pop() {
            // Careful with the conditions here, so node1 == node2 works
            if current.value == node2 && !backref.is_empty() {
                info!("Found a path");
                let mut path = Vec::new();
                let mut at = current.value;
                loop {
                    path.push(at);
                    at = backref[&at];
                    if at == node1 {
                        path.push(at);
                        path.reverse();
                        assert_eq!(path[0], node1);
                        assert_eq!(*path.last().unwrap(), node2);

                        // TODO Hack to make progress
                        if path.len() > 3 {
                            return Some(path);
                        } else {
                            break;
                        }
                    }
                }
            }
            if visited.contains(&current.value) {
                continue;
            }
            visited.insert(current.value);

            for edge in &self.nodes[&current.value].edges {
                let edge = &self.edges[edge];
                // Quick rounding for Ordness
                let cost = current.cost + (edge.linestring.euclidean_length() * 100.0) as usize;
                let next = edge.other_node(current.value);
                //info!("get from {:?} to {:?} for total cost {}", current.value, next, cost);
                queue.push(PriorityQueueItem {
                    cost,
                    value: next,
                });
                backref.insert(next, current.value);
            }
        }

        None
    }
}
