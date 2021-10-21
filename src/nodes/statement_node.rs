use super::attributes::{AttributeListNode, AttributeStatementNode};
use super::common::{AssignmentStatementNode, NodeID};
use super::edge_statement::EdgeStatementNode;
use super::graph_node::SubgraphNode;

#[derive(Clone, Debug)]
pub enum StatementNode {
    Node(NodeStatementNode),
    Edge(EdgeStatementNode),
    Attribute(AttributeStatementNode),
    Assignment(AssignmentStatementNode),
    Subgraph(SubgraphNode),
}

#[derive(Clone, Debug)]
pub struct NodeStatementNode {
    id: NodeID,
    attributes: Vec<AttributeListNode>,
}
