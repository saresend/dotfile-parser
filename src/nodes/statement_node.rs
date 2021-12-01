use super::attributes::{AttributeListNode, AttributeStatementNode};
use super::common::{AssignmentStatementNode, NodeID};
use super::edge_statement::EdgeStatementNode;
use super::graph_node::SubgraphNode;

use crate::lex::{Peekable, Token};
use crate::parse::DotParseable;
use anyhow::{anyhow, Result};

#[derive(Clone, Debug)]
pub enum StatementNode {
    Node(NodeStatementNode),
    Edge(EdgeStatementNode),
    Attribute(AttributeStatementNode),
    Assignment(AssignmentStatementNode),
    Subgraph(SubgraphNode),
}
impl DotParseable for StatementNode {
    fn from_lexer<'a>(
        token_stream: &mut (impl Iterator<Item = Token> + Peekable<'a, Item = Token> + Clone),
    ) -> Result<Self> {
        if let Some(token) = token_stream.peek() {
            match token {
                Token::ID => {
                    let _id_token = token_stream.next();
                    match token_stream.peek() {
                        Some(&Token::DirectedEdge) => {
                            let edge_statement = EdgeStatementNode::from_lexer(token_stream)?;
                            Ok(StatementNode::Edge(edge_statement))
                        }
                        Some(&Token::UndirectedEdge) => {
                            let edge_statement = EdgeStatementNode::from_lexer(token_stream)?;
                            Ok(StatementNode::Edge(edge_statement))
                        }
                        Some(&Token::OpenBracket) => {
                            let node_statement = NodeStatementNode::from_lexer(token_stream)?;
                            Ok(StatementNode::Node(node_statement))
                        },
                        _ => Err(anyhow!("Syntax Error: unexpected token")),
                    }
                }
                Token::Graph | Token::Node | Token::Edge => {
                    let attr_stmt = AttributeStatementNode::from_lexer(token_stream)?;
                    Ok(StatementNode::Attribute(attr_stmt))
                },
                _ => Err(anyhow!("Syntax Error; unexpected token")),
            }
        } else {
            Err(anyhow!("Unexpected EOF, token not found"))
        }
    }
}

impl StatementNode {}

#[derive(Clone, Debug)]
pub struct NodeStatementNode {
    id: NodeID,
    attributes: Vec<AttributeListNode>,
}

impl DotParseable for NodeStatementNode {
    fn from_lexer<'a>(
        token_stream: &mut (impl Iterator<Item = Token> + Peekable<'a>),
    ) -> Result<Self> {
        todo!()
    }
}
