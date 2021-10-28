use super::common::IDNode;
use super::statement_node::StatementNode;
use crate::lex::Token;
use crate::parse::DotParseable;
use anyhow::Result;

#[derive(Clone, Debug)]
pub struct GraphNode {
    statements: Vec<StatementNode>,
}

impl DotParseable for GraphNode {
    fn from_lexer(
        token_stream: &mut impl Iterator<Item = Token>,
    ) -> Result<Self> {
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
    fn from_lexer(tstream: &mut impl Iterator<Item = Token>) -> Result<Self> {
        while let Ok(statment) = StatementNode::from_lexer(tstream) {
            let c_token = tstream.next();
        }
        todo!()
    }
    
}

#[derive(Clone, Debug)]
pub struct SubgraphNode {
    id: Option<IDNode>,
    statements: Vec<StatementNode>,
}
