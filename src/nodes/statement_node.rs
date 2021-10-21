use super::edge_statement::EdgeStatementNode;
use super::attributes::{AttributeStatementNode, AttributeListNode};
use super::common::{ NodeID, AssignmentStatementNode};
use super::graph_node::SubgraphNode;

#[derive(Clone, Debug)]
pub enum StatementNode {
    NodeStatement(NodeStatementNode),
    EdgeStatement(EdgeStatementNode),
    AttributeStatement(AttributeStatementNode),
    AssignmentStatement(AssignmentStatementNode),
    SubgraphStatement(SubgraphNode),
}

#[derive(Clone, Debug)]
pub struct NodeStatementNode {
    id: NodeID,
    attributes: Vec<AttributeListNode>,
}
