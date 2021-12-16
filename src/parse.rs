use super::lex::{Peekable, PeekableLexer};
use crate::lex::Token;
use anyhow::Result;

pub trait DotASTNode: RecurseDebug + DotParseable {}
impl<T> DotASTNode for T where T: RecurseDebug + DotParseable {}

pub trait DotParseable {
    fn from_lexer<'a>(
        token_stream: &mut (impl Iterator<Item = Token> + Peekable<'a, Item = Token> + Clone),
    ) -> Result<Self>
    where
        Self: Sized;
}

pub trait RecurseDebug {
    fn rec_fmt(&self, f: &mut std::fmt::Formatter<'_>, indent_level: usize) -> Result<(), anyhow::Error>;
}
