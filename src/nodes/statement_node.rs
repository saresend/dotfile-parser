use super::attributes::{AttributeListNode, AttributeStatementNode};
use super::common::{AssignmentStatementNode, NodeID};
use super::edge_statement::EdgeStatementNode;
use super::graph_node::SubgraphNode;

use crate::lex::{Token, Peekable};
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
    fn from_lexer(token_stream: &mut (impl Iterator<Item = Token> + Peekable<Item = Token> + Clone)) -> Result<Self> {
        if let Some(token) = token_stream.peek()  {
            match token {
                Token::ID => { 
                    let node_option = NodeStatementNode::from_lexer(&mut token_stream.clone());
                    if let Ok(node_option) = node_option {
                        return Ok(Self::Node(node_option));
                    } else {
                        let edge_stmnt = EdgeStatementNode::from_lexer(&mut token_stream.clone()); 
                    }
                    todo!() 
                },
                _ => Err(anyhow!("Syntax Error; unexpected token"))
            }
        } else {
            Err(anyhow!("Unexpected EOF, token not found"))
        }
    }
}

#[derive(Clone, Debug)]
pub struct NodeStatementNode {
    id: NodeID,
    attributes: Vec<AttributeListNode>,
}


impl DotParseable for NodeStatementNode {
    fn from_lexer(token_stream: &mut(impl Iterator<Item = Token> + Peekable)) -> Result<Self> {
        todo!()
    }
}
