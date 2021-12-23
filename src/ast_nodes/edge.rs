use super::assignment::AttributeList;
use super::node::Node;
use super::ID;
use std::marker::PhantomData;

use crate::lex::{Peekable, Token};

use crate::parse::Constructable;

struct Directed;
struct Undirected;

pub struct Edge<LHS, T> {
    lhs: LHS,
    ty: PhantomData<T>,
    attr_list: AttributeList,
}

impl Constructable for Edge<Node, Directed> {
    fn from_lexer(
        mut token_stream: crate::lex::PeekableLexer,
    ) -> anyhow::Result<(Self, crate::lex::PeekableLexer), anyhow::Error> {
        if let Some(Token::ID) = token_stream.next() {
            let node_id = token_stream.slice().to_string();
            if let Some(Token::DirectedEdge) = token_stream.next() {
                todo!()
            } else {
                Err(anyhow::anyhow!("Invalid token; expected directed edge"))
            }
        } else {
            Err(anyhow::anyhow!("Invalid token; expected ID"))
        }
    }
}
