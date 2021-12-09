use super::attributes::AttributeListNode;
use super::common::NodeID;
use super::edge_rhs::EdgeRHSNode;
use super::graph_node::SubgraphNode;
use crate::lex::{Peekable, Token};
use crate::parse::DotParseable;

use anyhow::anyhow;

#[derive(Clone, Debug)]
pub enum EdgeStatementNode {
    Node((NodeID, EdgeRHSNode, Vec<AttributeListNode>)),
    Subgraph((SubgraphNode, EdgeRHSNode, Vec<AttributeListNode>)),
}

impl DotParseable for EdgeStatementNode {
    fn from_lexer<'a>(
        token_stream: &mut (impl Iterator<Item = Token> + Peekable<'a, Item = Token> + Clone),
    ) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let c_token = token_stream.peek();     
        match c_token {
            Some(x) if x == &Token::DirectedEdge => {
                token_stream.next();
                todo!()
            },
            Some(x) if x == &Token::UndirectedEdge => {
                todo!()
            }
            _ => {Err(anyhow!("Syntax Error: Unexpected token")) }

        }
    }
}
