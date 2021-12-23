
use super::lex::{Peekable, PeekableLexer};
use crate::lex::Token;
use anyhow::Result;

pub trait Constructable: Sized {
    fn from_lexer(token_stream: PeekableLexer) -> Result<(Self, PeekableLexer), anyhow::Error>;
}

struct ParseOR<T, V> {
    t_val: Option<T>,
    v_val: Option<V>,
}

impl<T, V> Constructable for ParseOR<T, V> where T: Constructable, V: Constructable {
    fn from_lexer(token_stream: PeekableLexer) -> Result<(Self, PeekableLexer), anyhow::Error> {
       if let  Ok((val, tok_s)) = T::from_lexer(token_stream.clone()) {
            Ok((Self { t_val: Some(val), v_val:None}, tok_s))
       } else if let Ok((val, tok_s)) = V::from_lexer(token_stream.clone()) {
            Ok((Self { t_val: None, v_val: Some(val) }, tok_s))
       } else {
        Err(anyhow::anyhow!("Invalid or expression; neither arm parsed successfully"))
       }
    }

}






