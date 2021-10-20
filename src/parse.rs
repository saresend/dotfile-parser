use crate::ast_nodes::*;
use crate::lex::Token;
use logos::Logos;
use petgraph::data::Build;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::marker::PhantomData;

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

    pub fn parse_into_graph(&mut self) -> Result<B, Box<dyn std::error::Error>> {
        let mut parse_str = self.get_token_string();
        let mut token_stream = Token::lexer(&mut parse_str);
        while let Some(curr_token) = token_stream.next() {
            match curr_token {
                Token::Strict => {
                    let r = GraphNode::parse_from_tks(&mut token_stream)?;
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
