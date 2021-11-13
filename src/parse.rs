use super::lex::{Peekable, PeekableLexer};
use crate::lex::Token;
use crate::nodes::graph_node::GraphNode;
use anyhow::Result;
use logos::Logos;
use petgraph::data::Build;
use std::io::BufReader;
use std::io::Read;
use std::marker::PhantomData;

pub trait DotParseable {
    fn from_lexer<'a>(
        token_stream: &mut (impl Iterator<Item = Token> + Peekable<'a, Item = Token> + Clone),
    ) -> Result<Self>
    where
        Self: Sized;
}

pub struct DotParser<R, B>
where
    R: std::io::Read,
{
    input: BufReader<R>,
    _output: PhantomData<B>,
}

impl<R, B> DotParser<R, B>
where
    R: std::io::Read,
    B: Build + Default,
{
    pub fn new(reader: R) -> Self {
        Self {
            input: BufReader::new(reader),
            _output: PhantomData,
        }
    }

    pub fn parse_into_graph(&mut self) -> Result<B> {
        let parse_str = self.get_token_string();
        let mut token_stream = PeekableLexer::new(Token::lexer(&parse_str));
        while let Some(curr_token) = token_stream.next() {
            match curr_token {
                Token::Strict => {
                    let r = GraphNode::from_lexer(&mut token_stream)?;
                }
                Token::Graph => {}
                Token::Digraph => {}
                _ => {}
            }
        }

        Ok(B::default())
    }

    fn get_token_string(&mut self) -> String {
        let mut s = String::new();
        self.input.read_to_string(&mut s);
        s
    }
}
