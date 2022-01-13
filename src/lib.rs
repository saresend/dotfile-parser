
//! A library for parsing graphviz dotfiles into an AST format 
//! 
//! This crate provides a basic parsing capability for the graphviz file format 
//! specified here: https://graphviz.org/doc/info/lang.html 
//!
//! What this crate provides is an AST format, and a parser to translate from graphviz files 
//! into that ast format. This means that this crate doesn't do all that much on its own, as 
//! it still takes some effort to then utilize the AST format rather than the raw graphviz file 
//! strings themselves. 
//!
//! To get started, imagine you have some graphviz file you'd like to work with that 
//! might look something like:
//! ```
//! digraph D { 
//!     A -> { B D } -> C [color=red];
//! }
//!
//! ```
//!
//! To parse this using this dotfile parser 
//!
//!
//!

use ast_nodes::{Undirected, Directed};
use lex::PeekableLexer;
use parse::{Constructable, ParseOR};
pub mod ast_nodes;

mod lex;
mod parse;

/// DotGraph is the toplevel graph construct we parse into.
///
/// Dotgraph can either be a directed graph, or an undirected graph, 
/// depending on the string input it is provided
///
/// **Note:** at the time of writing, this does not handle
/// strict support.
pub enum DotGraph {
    Undirected(Box<ast_nodes::Graph<Undirected>>),
    Directed(Box<ast_nodes::Graph<Directed>>),
}


impl std::str::FromStr for DotGraph {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let peekable_lexer = PeekableLexer::from(s);
        let dir_or_undir = ParseOR::<ast_nodes::Graph<Directed>, ast_nodes::Graph<Undirected>>::from_lexer(peekable_lexer)?.0;
        match dir_or_undir {
            ParseOR { t_val: None, v_val: Some(undirect) } => {
                Ok(Self::Undirected(Box::new(undirect)))
            },
            ParseOR { t_val: Some(direct), v_val: None } => {
                Ok(Self::Directed(Box::new(direct)))
            },
            _ => Err(anyhow::anyhow!("Error; couldn't parse as either directed or undirected graph")),
        }
    }

}

