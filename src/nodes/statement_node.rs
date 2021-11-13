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
    fn from_lexer<'a>(token_stream: &mut (impl Iterator<Item = Token> + Peekable<'a, Item = Token> + Clone)) -> Result<Self> {
        if let Some(token) = token_stream.peek()  {
            match token {
                Token::ID => { todo!() } ,
                _ => Err(anyhow!("Syntax Error; unexpected token"))
            }
        } else {
            Err(anyhow!("Unexpected EOF, token not found"))
        }
    }
}

impl StatementNode {


}

#[derive(Clone, Debug)]
pub struct NodeStatementNode {
    id: NodeID,
    attributes: Vec<AttributeListNode>,
}


impl DotParseable for NodeStatementNode {
    fn from_lexer<'a>(token_stream: &mut(impl Iterator<Item = Token> + Peekable<'a>)) -> Result<Self> {
        todo!()
    }
}
