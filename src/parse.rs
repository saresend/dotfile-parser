use super::lex::{Peekable, PeekableLexer};
use crate::lex::Token;
use anyhow::Result;


pub trait Constructable: Sized {
    fn from_lexer(token_stream: PeekableLexer) -> Result<(Self, PeekableLexer), anyhow::Error>;
}


