use super::attributes::{AttributeListNode, AttributeStatementNode};
use super::common::{AssignmentStatementNode, NodeID};
use super::edge_statement::EdgeStatementNode;
use super::graph_node::SubgraphNode;

use crate::lex::{Peekable, Token};
use crate::parse::{RecurseDebug, DotParseable};
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
                        Some(&Token::Equals) => {
                            let assign_stmt = AssignmentStatementNode::from_lexer(token_stream)?;
                            Ok(StatementNode::Assignment(assign_stmt))
                        },
                        _ => Err(anyhow!("Syntax Error: unexpected token")),
                    }
                }
                Token::Graph | Token::Node | Token::Edge => {
                    let attr_stmt = AttributeStatementNode::from_lexer(token_stream)?;
                    Ok(StatementNode::Attribute(attr_stmt))
                },
                Token::Subgraph | Token::OpenParen => {
                    let subgraph_stmt = SubgraphNode::from_lexer(token_stream)?;
                    Ok(StatementNode::Subgraph(subgraph_stmt)) 
                }

                _ => Err(anyhow!("Syntax Error; unexpected token")),
            }
        } else {
            Err(anyhow!("Unexpected EOF, token not found"))
        }
    }
}

impl RecurseDebug for StatementNode {

    fn rec_fmt(&self, f: &mut std::fmt::Formatter<'_>, indent_level: usize) -> Result<(), anyhow::Error> {
        let mut indent_str = String::new();
        for _ in 0..indent_level { 
            indent_str += " ";
        }

        f.write_str(&indent_str)?;

        match self {
            StatementNode::Node(inner_node) => inner_node.rec_fmt(f, indent_level + 1),
            _ => Ok(()),
            /*
            StatementNode::Edge(innerEdge) => innerEdge.rec_fmt(f, indent_level + 1), 
            StatementNode::Attribute(innerAttribute) => rec_fmt(f, indent_level + 1), 
            StatementNode::Assignment(innerAssignment) => rec_fmt(f, indent_level + 1), 
            StatementNode::Subgraph(innerSubgraph) => rec_fmt(f, indent_level + 1), 
            */
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

impl RecurseDebug for NodeStatementNode {

    fn rec_fmt(&self, f: &mut std::fmt::Formatter<'_>, indent_level: usize) -> Result<(), anyhow::Error> {
        let mut indent_str = String::new();
        for _ in 0..indent_level {
            indent_str += " ";
        }
        f.write_str(&indent_str)?;
        f.write_str(&self.id.id)?;
        f.write_str("\n")?;

        if let Some(ref port_num) = self.id.port {
            f.write_str(&indent_str)?;
            f.write_str(port_num)?;
        }
    }
}
