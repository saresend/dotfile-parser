use super::common::NodeID;
use super::edge_operation::EdgeOP;

use super::graph_node::SubgraphNode;

#[derive(Clone, Debug)]
pub enum EdgeRHSNode {
    Node((EdgeOP, NodeID, Box<Option<EdgeRHSNode>>)),
    Subgraph(SubgraphNode, Box<Option<EdgeRHSNode>>),
}
