use super::assignment::AttributeList;
use super::node::Node;
use super::Subgraph;
use super::ID;
use std::marker::PhantomData;

use crate::lex::{Peekable, Token};

use crate::parse::{Constructable, ParseOR};

struct Directed;
struct Undirected;

enum EdgeLHS {
    Node(Node),
    Subgraph(Subgraph),
}

pub struct Edge<T> {
    lhs: EdgeLHS,
    rhs: Box<Edge<T>>,
    ty: PhantomData<T>,
    attr_list: AttributeList,
}

impl Constructable for Edge<Directed> {
    type Output = Self;

    fn from_lexer(
        token_stream: crate::lex::PeekableLexer,
    ) -> anyhow::Result<(Self::Output, crate::lex::PeekableLexer), anyhow::Error> {
        let mut p1 = ParseOR::<Node, Subgraph>::from_lexer(token_stream.clone())?;
        if let Some(lhs) = p1.0.t_val {
            if let Some(Token::DirectedEdge) = p1.1.next() {
                let rhs = Edge::<Directed>::from_lexer(p1.1)?;
                let cloned_attr = rhs.0.attr_list.clone();
                Ok((
                    Self {
                        lhs: EdgeLHS::Node(lhs),
                        rhs: Box::new(rhs.0),
                        ty: PhantomData,
                        attr_list: cloned_attr,
                    },
                    rhs.1,
                ))
            } else {
                Err(anyhow::anyhow!("Invalid token following node id for edge"))
            }
        } else if let Some(lhs) = p1.0.v_val {
            let rhs = Edge::<Directed>::from_lexer(p1.1)?;
            let cloned_attr = rhs.0.attr_list.clone();

            Ok((
                Self {
                    lhs: EdgeLHS::Subgraph(lhs),
                    rhs: Box::new(rhs.0),
                    ty: PhantomData,
                    attr_list: cloned_attr,
                },
                rhs.1,
            ))
        } else {
            Err(anyhow::anyhow!(
                "Couldn't parse either Node or Subgraph for Edge Node"
            ))
        }
    }
}
