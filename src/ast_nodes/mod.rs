//!
//! The AST nodes module provides all the ast components
//! that comprise a graphviz file. These node types
//! map quite closely to the graphviz documentation provided
//! here: [link](https://graphviz.org/doc/info/lang.html)
//!
//! These components provide a better structure for operating on
//! graph structures. For example, the following code lets us
//! list all of the node identifiers that exist inside of a graph:
//!
//! ```
//! use graphviz_parser::DotGraph;
//! use graphviz_parser::ast_nodes::Statement::Node;
//! use std::str::FromStr;
//!
//! let dot_graph = DotGraph::from_str("graph G { a; b; c; }").unwrap();
//! let mut node_ids = vec![];
//! if let DotGraph::Directed(graph) = dot_graph {
//!    for statement in graph.statements {
//!       if let Node(n) = statement {
//!             node_ids.push(n.id);
//!       }
//!    }
//!    assert_eq!(node_ids, vec!["a", "b", "c"]);
//! }
//! ```

mod assignment;
mod edge;
mod node;
mod statement;
mod subgraph;

use std::marker::PhantomData;

pub use assignment::Assignment;
pub use edge::{Directed, Undirected};
pub use edge::{Edge, EdgeLHS, EdgeRHS};
pub use node::Node;
pub use statement::Statement;
pub use subgraph::Subgraph;

/// An ID represents any identifier used inside
/// of a graphviz file. This could represent an attribute
/// name, an attribute value, a graph or sugraph name, and node names
pub type ID = String;

use crate::lex::{Peekable, PeekableLexer, Token};
use crate::parse::Constructable;

impl Constructable for ID {
    type Output = Self;
    fn from_lexer(
        mut token_stream: crate::lex::PeekableLexer,
    ) -> anyhow::Result<(Self::Output, crate::lex::PeekableLexer), anyhow::Error> {
        if let Some(Token::ID) = token_stream.next() {
            Ok((token_stream.slice().to_owned(), token_stream))
        } else {
            Err(anyhow::anyhow!("Expected type ID"))
        }
    }
}

///
/// A graph is the underlying structure that represents a toplevel graph in graphviz
/// this should roughly correspond to the graph production
/// [here](https://graphviz.org/doc/info/lang.html)
///
/// Example usage of this:
///```
/// use graphviz_parser::DotGraph;
/// use graphviz_parser::ast_nodes::Statement::Node;
/// use std::str::FromStr;
/// let dot_graph = DotGraph::from_str("graph G { a; b; c; }").unwrap();
/// if let DotGraph::Directed(graph) = dot_graph {
///    assert_eq!(graph.id, String::from("G"));
/// }
/// ```
///
///
#[derive(Debug)]
pub struct Graph<T> {
    pub id: ID,
    pub is_strict: bool,
    pub statements: Vec<Statement<T>>,
    _pd: PhantomData<T>,
}

impl Constructable for Graph<Directed> {
    type Output = Self;
    fn from_lexer(
        mut token_stream: PeekableLexer,
    ) -> anyhow::Result<(Self::Output, PeekableLexer), anyhow::Error> {
        let mut is_strict = false;
        if token_stream.peek() == Some(&Token::Strict) {
            token_stream.next();
            is_strict = true;
        }
        match token_stream.next() {
            Some(Token::Digraph) => {
                match (
                    token_stream.next(),
                    String::from(token_stream.slice()),
                    token_stream.next(),
                ) {
                    (Some(Token::ID), graph_id, Some(Token::OpenParen)) => {
                        let (statements, tstream) =
                            Vec::<Statement<Directed>>::from_lexer(token_stream)?;
                        Ok((
                            Self {
                                id: graph_id,
                                statements,
                                is_strict,
                                _pd: PhantomData,
                            },
                            tstream,
                        ))
                    }
                    _ => {
                        todo!()
                    }
                }
            }
            _ => Err(anyhow::anyhow!("Error; invalid start token")),
        }
    }
}

impl Constructable for Graph<Undirected> {
    type Output = Self;
    fn from_lexer(
        mut token_stream: PeekableLexer,
    ) -> anyhow::Result<(Self::Output, PeekableLexer), anyhow::Error> {
        let mut is_strict = false;
        if token_stream.peek() == Some(&Token::Strict) {
            token_stream.next();
            is_strict = true;
        }

        match token_stream.next() {
            Some(Token::Graph) => {
                match (
                    token_stream.next(),
                    String::from(token_stream.slice()),
                    token_stream.next(),
                ) {
                    (Some(Token::ID), graph_id, Some(Token::OpenParen)) => {
                        let (statements, tstream) =
                            Vec::<Statement<Undirected>>::from_lexer(token_stream)?;
                        Ok((
                            Self {
                                id: graph_id,
                                statements,
                                is_strict,
                                _pd: PhantomData,
                            },
                            tstream,
                        ))
                    }
                    _ => {
                        todo!()
                    }
                }
            }
            _ => Err(anyhow::anyhow!("Error; invalid start token")),
        }
    }
}
