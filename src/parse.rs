use super::lex::{Peekable, PeekableLexer};
use crate::lex::Token;
use anyhow::Result;

pub trait Constructable: Sized {
    type Output;
    fn from_lexer(
        token_stream: PeekableLexer,
    ) -> Result<(Self::Output, PeekableLexer), anyhow::Error>;
}

pub struct ParseOR<T: Constructable, V: Constructable> {
    pub t_val: Option<T::Output>,
    pub v_val: Option<V::Output>,
}

impl<T, V> Constructable for ParseOR<T, V>
where
    T: Constructable,
    V: Constructable,
{
    type Output = ParseOR<T, V>;

    fn from_lexer(token_stream: PeekableLexer) -> Result<(Self, PeekableLexer), anyhow::Error> {
        if let Ok((val, tok_s)) = T::from_lexer(token_stream.clone()) {
            Ok((
                Self {
                    t_val: Some(val),
                    v_val: None,
                },
                tok_s,
            ))
        } else if let Ok((val, tok_s)) = V::from_lexer(token_stream.clone()) {
            Ok((
                Self {
                    t_val: None,
                    v_val: Some(val),
                },
                tok_s,
            ))
        } else {
            Err(anyhow::anyhow!(
                "Invalid or expression; neither arm parsed successfully"
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Constructable;
    use super::ParseOR;
    use crate::ast_nodes::{Assignment, Node};
    use crate::lex::PeekableLexer;

    #[test]
    fn or_op_sanity_test1() {
        let test_str = "color = green";
        let pb = PeekableLexer::from(test_str);
        let result: ParseOR<Assignment, Node> = ParseOR::from_lexer(pb).unwrap().0;
        assert!(result.t_val.is_some());
        assert_eq!(result.t_val.unwrap(), Assignment::new("color", "green"));
    }
}
