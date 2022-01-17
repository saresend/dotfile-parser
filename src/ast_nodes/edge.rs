use super::assignment::AttributeList;
use super::Subgraph;
use super::ID;
use std::marker::PhantomData;

use crate::lex::Token;

use crate::parse::{Constructable, ParseOR};

pub(crate) trait GraphDirection {
    fn token() -> Token;
}

///
/// Directed is a marker type that is used
/// as a paremeter for types that have an associated direction
///
/// As an example, a graph can either be directed or undirected, and
/// similarly an edge can also have a direction.
///
#[derive(Debug)]
pub struct Directed;

impl GraphDirection for Directed {
    fn token() -> Token {
        Token::DirectedEdge
    }
}

#[derive(Debug)]
pub struct Undirected;

impl GraphDirection for Undirected {
    fn token() -> Token {
        Token::UndirectedEdge
    }
}

#[derive(Debug)]
pub enum EdgeLHS<T> {
    Node(ID),
    Subgraph(Subgraph<T>),
}

impl<T: GraphDirection> Constructable for EdgeLHS<T> {
    type Output = Self;

    fn from_lexer(
        token_stream: crate::lex::PeekableLexer,
    ) -> anyhow::Result<(Self::Output, crate::lex::PeekableLexer), anyhow::Error> {
        let option = ParseOR::<Subgraph<T>, ID>::from_lexer(token_stream.clone())?;
        match option {
            (
                ParseOR {
                    t_val: None,
                    v_val: Some(id),
                },
                tok_s,
            ) => Ok((EdgeLHS::Node(id), tok_s)),
            (
                ParseOR {
                    t_val: Some(subgraph),
                    v_val: None,
                },
                tok_s,
            ) => Ok((EdgeLHS::Subgraph(subgraph), tok_s)),
            _ => Err(anyhow::anyhow!("Couldn't parse Edge LHS")),
        }
    }
}

#[derive(Debug)]
pub enum EdgeRHS<T> {
    Edge(Edge<T>),
    Node(ID),
    Subgraph(Subgraph<T>),
}

impl<T: GraphDirection> Constructable for EdgeRHS<T> {
    type Output = Self;

    fn from_lexer(
        token_stream: crate::lex::PeekableLexer,
    ) -> anyhow::Result<(Self::Output, crate::lex::PeekableLexer), anyhow::Error> {
        let (options, token_stream) =
            ParseOR::<Edge<T>, ParseOR<ID, Subgraph<T>>>::from_lexer(token_stream)?;
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

///
/// An Edge represents the top level struct that contains information about 
/// an edge representation. An Edge has a Direction (i.e. directed or undirected), 
/// as well as the left hand side, that can either be a node, or a subgraph, as 
/// well a rhs, that is either another edge, a node, or a subgraph. 
///
/// One thing worth noting is that graphviz represents edges that connect 
/// two subraphs to represent edges from *all* nodes defined inside the left
/// sugraph, to *all* nodes inside the right subgraph
/// 
/// For example, the graph `{A B} -> {D E}` defines the following edges:
/// (A, D), (A,E), (B, D), (B, E)
///
///
///
#[derive(Debug)]
pub struct Edge<T> {
    pub lhs: EdgeLHS<T>,
    pub rhs: Box<EdgeRHS<T>>,
    ty: PhantomData<T>,
    pub attr_list: AttributeList,
}

impl<T: GraphDirection> Constructable for Edge<T> {
    type Output = Self;

    fn from_lexer(
        token_stream: crate::lex::PeekableLexer,
    ) -> anyhow::Result<(Self::Output, crate::lex::PeekableLexer), anyhow::Error> {
        let (lhs, mut token_stream) = EdgeLHS::<T>::from_lexer(token_stream)?;
        if Some(T::token()) == token_stream.next() {
            let (rhs, mut token_stream) = EdgeRHS::<T>::from_lexer(token_stream)?;
            let mut attributes = vec![];
            match AttributeList::from_lexer(token_stream.clone()) {
                Ok((attr_list, t_stream)) => {
                    attributes = attr_list;
                    token_stream = t_stream;
                }
                Err(_) => {} // TODO: Address issues
            };

            Ok((
                Self {
                    lhs,
                    rhs: Box::new(rhs),
                    ty: PhantomData,
                    attr_list: attributes,
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
    fn edge_directed_attribute_test() {
        let test_str = "A -> B [color=green, shape=circle]";
        let pb = PeekableLexer::from(test_str);
        let res = Edge::<Directed>::from_lexer(pb).unwrap().0;
        assert_eq!(res.attr_list[0].len(), 2);
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
