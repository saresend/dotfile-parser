use super::common::IDNode;
use crate::parse::DotParseable;

use anyhow;

use crate::lex::{Peekable, Token};

#[derive(Clone, Debug)]
pub struct AttributeNode {
    lhs_id: IDNode,
    rhs_id: IDNode,
}

#[derive(Clone, Debug)]
pub struct AttributeListNode {
    attributes: Vec<AttributeNode>,
}

#[derive(Clone, Debug)]
pub enum AttributeStatementNode {
    Graph(Vec<AttributeListNode>),
    Node(Vec<AttributeListNode>),
    Edge(Vec<AttributeListNode>),
}


impl DotParseable for AttributeStatementNode {
    fn from_lexer<'a>(token_stream: &mut (impl Iterator<Item = Token> + Peekable<'a, Item = Token> + Clone)) -> Result<Self, anyhow::Error>{
        todo!()
    }
}
