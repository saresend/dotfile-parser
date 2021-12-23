use super::assignment::AttributeList;
use super::node::Node;
use super::Subgraph;
use super::ID;
use std::marker::PhantomData;

use crate::lex::{Peekable, Token};

use crate::parse::{Constructable, ParseOR};

struct Directed;
struct Undirected;

pub struct Edge<LHS, T> {
    lhs: LHS,
    ty: PhantomData<T>,
    attr_list: AttributeList,

}

impl Constructable for Edge<ID, Directed> {
    type Output = Edge::<Node, Directed>;

    fn from_lexer(
        mut token_stream: crate::lex::PeekableLexer,
    ) -> anyhow::Result<(Self::Output, crate::lex::PeekableLexer), anyhow::Error> {
        todo!()
    }
}

impl Constructable for Edge<Subgraph, Directed> {
    type Output = Edge<Subgraph, Directed>;
    
    fn from_lexer(
        token_stream: crate::lex::PeekableLexer,
    ) -> anyhow::Result<(Self, crate::lex::PeekableLexer), anyhow::Error> {
        todo!()
    }
}
