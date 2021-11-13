use super::attributes::AttributeListNode;
use super::common::NodeID;
use super::edge_rhs::EdgeRHSNode;
use super::graph_node::SubgraphNode;
use crate::lex::{Token, Peekable};
use crate::parse::DotParseable;

#[derive(Clone, Debug)]
pub enum EdgeStatementNode {
    Node((NodeID, EdgeRHSNode, Vec<AttributeListNode>)),
    Subgraph((SubgraphNode, EdgeRHSNode, Vec<AttributeListNode>)),
}

impl DotParseable for EdgeStatementNode {

    fn from_lexer<'a>(token_stream: &mut (impl Iterator<Item = Token> + Peekable<'a, Item = Token> + Clone)) -> anyhow::Result<Self>
    where
            Self: Sized {
        todo!()
    }
}
