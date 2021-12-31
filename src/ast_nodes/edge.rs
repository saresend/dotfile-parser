use super::assignment::AttributeList;
use super::Subgraph;
use super::ID;
use std::marker::PhantomData;

use crate::lex::Token;

use crate::parse::{Constructable, ParseOR};

struct Directed;
struct Undirected;

enum EdgeLHS {
    Node(ID),
    Subgraph(Subgraph),
}

impl Constructable for EdgeLHS {
    type Output = Self;

    fn from_lexer(
        token_stream: crate::lex::PeekableLexer,
    ) -> anyhow::Result<(Self::Output, crate::lex::PeekableLexer), anyhow::Error> {
        let option = ParseOR::<ID, Subgraph>::from_lexer(token_stream.clone())?;
        match option {
            (
                ParseOR {
                    t_val: None,
                    v_val: Some(subgraph),
                },
                tok_s,
            ) => Ok((EdgeLHS::Subgraph(subgraph), tok_s)),
            (
                ParseOR {
                    t_val: Some(id),
                    v_val: None,
                },
                tok_s,
            ) => Ok((EdgeLHS::Node(id), tok_s)),
            _ => Err(anyhow::anyhow!("Couldn't parse Edge LHS")),
        }
    }
}

enum EdgeRHS<T> {
    Edge(Edge<T>),
    Node(ID),
    Subgraph(Subgraph),
}

impl Constructable for EdgeRHS<Directed> {
    type Output = Self;

    fn from_lexer(
        token_stream: crate::lex::PeekableLexer,
    ) -> anyhow::Result<(Self::Output, crate::lex::PeekableLexer), anyhow::Error> {
        let (options, token_stream) =
            ParseOR::<Edge<Directed>, ParseOR<ID, Subgraph>>::from_lexer(token_stream)?;
        match options {
            ParseOR {
                t_val: Some(edge),
                v_val: None,
            } => Ok((EdgeRHS::Edge(edge), token_stream)),
            ParseOR {
                t_val: None,
                v_val:
                    Some(ParseOR {
                        t_val: Some(id),
                        v_val: None,
                    }),
            } => Ok((EdgeRHS::Node(id), token_stream)),
            ParseOR {
                t_val: None,
                v_val:
                    Some(ParseOR {
                        t_val: None,
                        v_val: Some(subgraph),
                    }),
            } => Ok((EdgeRHS::Subgraph(subgraph), token_stream)),
            _ => Err(anyhow::anyhow!("Couldn't construct EdgeRHS")),
        }
    }
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
        mut token_stream: crate::lex::PeekableLexer,
    ) -> anyhow::Result<(Self::Output, crate::lex::PeekableLexer), anyhow::Error> {
        let (lhs, mut token_stream) = EdgeLHS::from_lexer(token_stream)?;
        if let Some(Token::DirectedEdge) = token_stream.next() {
            let (rhs, token_stream) = EdgeRHS::from_lexer(token_stream)?;
            Ok((
                Self {
                    lhs,
                    rhs: Box::new(rhs),
                    ty: PhantomData,
                    attr_list: vec![],
                },
                token_stream,
            ))
        } else {
            Err(anyhow::anyhow!(
                "Couldn't find directed edge for Edge<Directed>"
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{EdgeLHS, EdgeRHS};
    use crate::ast_nodes::{edge::Directed, Edge};
    use crate::lex::PeekableLexer;
    use crate::parse::Constructable;

    #[test]
    fn edge_directed_statement_sanity_node_test() {
        let test_str = "A -> B";
        let pb = PeekableLexer::from(test_str);
        let res = Edge::<Directed>::from_lexer(pb).unwrap();
        let rhs_v = *res.0.rhs;
        let edg_lhs = res.0.lhs;
        if let EdgeRHS::<Directed>::Node(id) = rhs_v {
            assert_eq!("B", id);
        } else {
            unreachable!()
        };

        if let EdgeLHS::Node(id) = edg_lhs {
            assert_eq!("A", id);
        } else {
            unreachable!()
        };
    }

    #[test]
    fn edge_directed_statement_multi_node_test() {
        let test_str = "A -> B -> C -> D -> E";
        let pb = PeekableLexer::from(test_str);
        let res = Edge::<Directed>::from_lexer(pb).unwrap().0;
        if let EdgeLHS::Node(id) = res.lhs {
            assert_eq!(id, "A");
        } else {
            unreachable!()
        }

        if let EdgeRHS::<Directed>::Edge(inner_edg) = *res.rhs {
            if let EdgeLHS::Node(id) = inner_edg.lhs {
                assert_eq!("B", id);
            } else {
                unreachable!()
            }

            if let EdgeRHS::<Directed>::Edge(inner_edg2) = *inner_edg.rhs {
                if let EdgeLHS::Node(id) = inner_edg2.lhs {
                    assert_eq!("C", id);
                } else {
                    unreachable!();
                }
            }
        }
    }

    #[test]
    fn node_statement_subgraph1_test() {
        let test_str = "test1 {A, B} -> {C, D}";
        let pb = PeekableLexer::from(test_str);
        let edge = Edge::<Directed>::from_lexer(pb).unwrap().0;
        if let EdgeLHS::Subgraph(subgraph) = edge.lhs {
            assert_eq!(subgraph.id, Some(String::from("test1")));
        } else {
            unreachable!()
        }
    }
}
