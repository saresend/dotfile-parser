use super::common::IDNode;
use super::statement_node::StatementNode;
use crate::lex::{Peekable, Token};
use crate::parse::DotParseable;
use anyhow::{anyhow, Result};

#[derive(Clone, Debug)]
pub struct GraphNode {
    statements: Vec<StatementNode>,
}

impl DotParseable for GraphNode {
    fn from_lexer<'a>(token_stream: &mut (impl Iterator<Item = Token> + Peekable<'a, Item = Token> + Clone)) -> Result<Self> {
        let c = token_stream.next();
        if c == Some(Token::OpenParen) {
            let statements = Vec::<StatementNode>::from_lexer(token_stream)?;
            Ok(Self { statements })
        } else {
            todo!()
        }
    }
}

impl DotParseable for Vec<StatementNode> {
    fn from_lexer<'a>(tstream: &mut (impl Iterator<Item = Token> + Peekable<'a, Item = Token> + Clone)) -> Result<Self> {
        let mut statements = vec![];
        while let Ok(statement) = StatementNode::from_lexer(tstream) {
            let c_token = tstream.next();
            if c_token != Some(Token::SemiColon) {
                return Err(anyhow!("Invalid syntax, missing semicolon"));
            }
            statements.push(statement);
        }
        Ok(statements)
    }
}

#[derive(Clone, Debug)]
pub struct SubgraphNode {
    id: Option<IDNode>,
    statements: Vec<StatementNode>,
}
