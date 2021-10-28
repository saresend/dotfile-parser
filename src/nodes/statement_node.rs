use super::attributes::{AttributeListNode, AttributeStatementNode};
use super::common::{AssignmentStatementNode, NodeID};
use super::edge_statement::EdgeStatementNode;
use super::graph_node::SubgraphNode;

use crate::parse::DotParseable;
use crate::lex::Token;
use anyhow::Result;

#[derive(Clone, Debug)]
pub enum StatementNode {
    Node(NodeStatementNode),
    Edge(EdgeStatementNode),
    Attribute(AttributeStatementNode),
    Assignment(AssignmentStatementNode),
    Subgraph(SubgraphNode),
}
impl DotParseable for StatementNode {
    fn from_lexer(
        token_stream: &mut impl Iterator<Item = Token>,
    ) -> Result<Self> {
        todo!()
    }
}

#[derive(Clone, Debug)]
pub struct NodeStatementNode {
    id: NodeID,
    attributes: Vec<AttributeListNode>,
}
