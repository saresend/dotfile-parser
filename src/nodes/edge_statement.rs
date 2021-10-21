use super::attributes::AttributeListNode;
use super::common::NodeID;
use super::edge_rhs::EdgeRHSNode;
use super::graph_node::SubgraphNode;

#[derive(Clone, Debug)]
pub enum EdgeStatementNode {
    Node((NodeID, EdgeRHSNode, Vec<AttributeListNode>)),
    Subgraph((SubgraphNode, EdgeRHSNode, Vec<AttributeListNode>)),
}
