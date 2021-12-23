use super::Statement;
use super::ID;

use crate::lex::Peekable;
use crate::parse::Constructable;

pub struct Subgraph {
    id: Option<ID>,
    statements: Vec<Statement>,
}

impl Constructable for Subgraph {
    type Output = Self;

    fn from_lexer(
        token_stream: crate::lex::PeekableLexer,
    ) -> anyhow::Result<(Self::Output, crate::lex::PeekableLexer), anyhow::Error> {
        todo!()
    }
}
