use super::assignment::AttributeList;
use super::Node;
use super::Subgraph;
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

///
/// Undirected is a marker type that is used
/// as a paremeter for types that have an associated direction
///
/// As an example, a graph can either be directed or undirected, and
/// similarly an edge can also have a direction.
///

#[derive(Debug)]
pub struct Undirected;

impl GraphDirection for Undirected {
    fn token() -> Token {
        Token::UndirectedEdge
    }
}

#[derive(Debug)]
pub enum EdgeLHS<T> {
    Node(Node),
    Subgraph(Subgraph<T>),
}

impl<T: GraphDirection> Constructable for EdgeLHS<T> {
    type Output = Self;

    fn from_lexer(
        token_stream: crate::lex::PeekableLexer,
    ) -> anyhow::Result<(Self::Output, crate::lex::PeekableLexer), anyhow::Error> {
        let option = ParseOR::<Subgraph<T>, Node>::from_lexer(token_stream.clone())?;
        match option {
            (
                ParseOR {
                    t_val: None,
                    v_val: Some(node),
                },
                tok_s,
            ) => Ok((EdgeLHS::Node(node), tok_s)),
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
    Node(Node),
    Subgraph(Subgraph<T>),
}

impl<T: GraphDirection> Constructable for EdgeRHS<T> {
    type Output = Self;

    fn from_lexer(
        token_stream: crate::lex::PeekableLexer,
    ) -> anyhow::Result<(Self::Output, crate::lex::PeekableLexer), anyhow::Error> {
        let (options, token_stream) =
            ParseOR::<Edge<T>, ParseOR<Node, Subgraph<T>>>::from_lexer(token_stream)?;
        match options {
            ParseOR {
                t_val: Some(edge),
                v_val: None,
            } => Ok((EdgeRHS::Edge(edge), token_stream)),
            ParseOR {
                t_val: None,
                v_val:
                    Some(ParseOR {
                        t_val: Some(node),
                        v_val: None,
                    }),
            } => Ok((EdgeRHS::Node(node), token_stream)),
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
            let (mut rhs, mut token_stream) = EdgeRHS::<T>::from_lexer(token_stream)?;
            let mut attributes = vec![];
            match AttributeList::from_lexer(token_stream.clone()) {
                Ok((attr_list, t_stream)) => {
                    attributes = attr_list;
                    token_stream = t_stream;
                }
                Err(_) => {} // TODO: Address issues
            };
            if let EdgeRHS::Edge(Edge {
                lhs: _,
                rhs: _,
                ty: _,
                attr_list: attribs,
            }) = &rhs
            {
                attributes = attribs.clone();
            } else if let EdgeRHS::Node(Node {
                id: _,
                port: _, 
                attribute_list: attribs,
            }) = &mut rhs {
                let moved_attributes = attribs.take();
                attributes = match moved_attributes {
                    Some(v) => {
                        v
                    },
                    None => vec![],
                };
            }
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
        if let EdgeRHS::<Directed>::Node(node) = rhs_v {
            assert_eq!("B", node.id);
        } else {
            unreachable!()
        };

        if let EdgeLHS::Node(node) = edg_lhs {
            assert_eq!("A", node.id);
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
        if let EdgeLHS::Node(node) = res.lhs {
            assert_eq!(node.id, "A");
        } else {
            unreachable!()
        }

        if let EdgeRHS::<Directed>::Edge(inner_edg) = *res.rhs {
            if let EdgeLHS::Node(node) = inner_edg.lhs {
                assert_eq!("B", node.id);
            } else {
                unreachable!()
            }

            if let EdgeRHS::<Directed>::Edge(inner_edg2) = *inner_edg.rhs {
                if let EdgeLHS::Node(node) = inner_edg2.lhs {
                    assert_eq!("C", node.id);
                } else {
                    unreachable!();
                }
            }
        }
    }

    #[test]
    fn edge_statement_subgraph1_test() {
        let test_str = "test1 {A, B} -> {C, D}";
        let pb = PeekableLexer::from(test_str);
        let edge = Edge::<Directed>::from_lexer(pb).unwrap().0;
        if let EdgeLHS::Subgraph(subgraph) = edge.lhs {
            assert_eq!(subgraph.id, Some(String::from("test1")));
        } else {
            unreachable!()
        }
    }

    #[test]
    fn edge_statement_compass_pt_node_test() {
        let test_str = "\"node0\":f0 -> \"node1\":f0 [
            id = 0
            ];
            ";
        let pb = PeekableLexer::from(test_str);
        let edge = Edge::<Directed>::from_lexer(pb).unwrap().0;
        if let EdgeLHS::Node(lhs) = edge.lhs{ 
            assert_eq!(lhs.id, String::from("\"node0\""));
            assert!(lhs.port.is_some());
        } else { unreachable!() }

        if let EdgeRHS::Node(rhs) = *edge.rhs {
            assert_eq!(rhs.id, String::from("\"node1\""));
            assert!(rhs.port.is_some());
        } else { unreachable!() } 
    }
}
