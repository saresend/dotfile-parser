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

impl Constructable for Edge<Node, Directed> {
    type Output = Self;

    fn from_lexer(
        mut token_stream: crate::lex::PeekableLexer,
    ) -> anyhow::Result<(Self, crate::lex::PeekableLexer), anyhow::Error> {
        if let Some(Token::ID) = token_stream.next() {
            let node_id = token_stream.slice().to_string();
            if let Some(Token::DirectedEdge) = token_stream.next() {
                let result = ParseOR::<Edge<Node, Directed>, Edge<Subgraph, Directed>>::from_lexer(
                    token_stream.clone(),
                )?;
                if let Some(val) = result.0.t_val {
                    // We'de like to return an Edge<Node, Directed>
                    todo!()
                } else if let Some(val) = result.0.v_val {
                    // We'd like to return an Edge<Subgraph, Directed>
                    todo!()
                }
                todo!()
            } else {
                Err(anyhow::anyhow!("Invalid token; expected directed edge"))
            }
        } else {
            Err(anyhow::anyhow!("Invalid token; expected ID"))
        }
    }
}

impl Constructable for Edge<Subgraph, Directed> {
    type Output = Self;
    
    fn from_lexer(
        token_stream: crate::lex::PeekableLexer,
    ) -> anyhow::Result<(Self, crate::lex::PeekableLexer), anyhow::Error> {
        todo!()
    }
}
