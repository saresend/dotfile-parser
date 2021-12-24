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
    Node(ID),
    Subgraph(Subgraph),
}

enum EdgeRHS<T> {
    Edge(Edge<T>),
    Node(ID),
    Subgraph(Subgraph),
}

pub struct Edge<T> {
    lhs: EdgeLHS,
    rhs: Box<EdgeRHS<T>>,
    ty: PhantomData<T>,
    attr_list: AttributeList,
}

impl Constructable for Edge<Directed> {
    type Output = Self;

    fn from_lexer(
        token_stream: crate::lex::PeekableLexer,
    ) -> anyhow::Result<(Self::Output, crate::lex::PeekableLexer), anyhow::Error> {
        let mut p1 = ParseOR::<ID, Subgraph>::from_lexer(token_stream.clone())?;
        if let Some(lhs) = p1.0.t_val {
            if let Some(Token::DirectedEdge) = p1.1.next() {
                let rhs = ParseOR::<Edge<Directed>, ParseOR<ID, Subgraph>>::from_lexer(p1.1)?;
                if let Some(v) = rhs.0.t_val {
                    Ok((
                        Self {
                            lhs: EdgeLHS::Node(lhs),
                            rhs: Box::new(EdgeRHS::Edge(v)),
                            ty: PhantomData,
                            attr_list: vec![],
                        },
                        rhs.1,
                    ))
                } else if let Some(inner) = rhs.0.v_val {
                    if let Some(id) = inner.t_val {
                    Ok((
                        Self {
                            lhs: EdgeLHS::Node(lhs),
                            rhs: Box::new(EdgeRHS::Node(id)),
                            ty: PhantomData,
                            attr_list: vec![],
                        },
                        rhs.1,
                    ))

                    } else if let Some(subgraph) = inner.v_val {
                    Ok((
                        Self {
                            lhs: EdgeLHS::Node(lhs),
                            rhs: Box::new(EdgeRHS::Subgraph(subgraph)),
                            ty: PhantomData,
                            attr_list: vec![],
                        },
                        rhs.1,
                    ))
                    } else {
                        Err(anyhow::anyhow!("Invalid token following node id for edge"))
                    }
                } else {
                    Err(anyhow::anyhow!("Invalid token following node id for edge"))
                }
            } else {
                Err(anyhow::anyhow!("Invalid token following node id for edge"))
            }
        } else if let Some(lhs) = p1.0.v_val {
            todo!()
        } else {
            Err(anyhow::anyhow!(
                "Couldn't parse either Node or Subgraph for Edge Node"
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ast_nodes::{edge::Directed, Edge};
    use crate::lex::PeekableLexer;
    use crate::parse::Constructable;
    use super::{EdgeLHS, EdgeRHS};

    #[test]
    fn edge_directed_statement_sanity_node_test1() {
        let test_str = "A -> B";
        let pb = PeekableLexer::from(test_str);
        let res = Edge::<Directed>::from_lexer(pb).unwrap();
        let rhs_v = *res.0.rhs;
        let edg_lhs = res.0.lhs;
        if let EdgeRHS::<Directed>::Node(id) = rhs_v {
            assert_eq!("B", id);
        } else { unreachable!() };

        if let EdgeLHS::Node(id) = edg_lhs{ 
            assert_eq!("A", id);
        } else { unreachable!() };
    }
}
