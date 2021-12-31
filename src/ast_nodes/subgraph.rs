use super::Statement;
use super::ID;

use crate::lex::{Peekable, Token};
use crate::parse::Constructable;

pub struct Subgraph {
    pub id: Option<ID>,
    pub statements: Vec<Statement>,
}

impl Constructable for Subgraph {
    type Output = Self;

    fn from_lexer(
        mut token_stream: crate::lex::PeekableLexer,
    ) -> anyhow::Result<(Self::Output, crate::lex::PeekableLexer), anyhow::Error> {
        if let Some(Token::ID) = token_stream.next() {
            let id = String::from(token_stream.slice());
            let (statements, tok_stream) = Vec::<Statement>::from_lexer(token_stream.clone())?;
            Ok((
                Self {
                    id: Some(id),
                    statements,
                },
                tok_stream,
            ))
        } else if let Some(Token::OpenParen) = token_stream.next() {
            let (statements, tok_stream) = Vec::<Statement>::from_lexer(token_stream.clone())?;
            Ok((
                Self {
                    id: None,
                    statements,
                },
                tok_stream,
            ))
        } else {
            Err(anyhow::anyhow!("Invalid construction of subgraph"))
        }
    }
}

#[cfg(test)]
mod tests {

    use super::Subgraph;
    use crate::lex::PeekableLexer;
    use crate::parse::Constructable;

    #[test]
    fn test_subgraph_sanity1_test() {
        let test_str = "test1 { A, B }";
        let pb = PeekableLexer::from(test_str);
        let subgraph = Subgraph::from_lexer(pb).unwrap().0;

        assert!(subgraph.id == Some(String::from("test1")));
        assert_eq!(subgraph.statements.len(), 2);
    }

    #[test]
    fn test_subgraph_sanity2_test() {
        let test_str = "test1 { A, B }";
        let pb = PeekableLexer::from(test_str);
        let subgraph = Subgraph::from_lexer(pb).unwrap().0;

        assert!(subgraph.id.is_none());
        assert_eq!(subgraph.statements.len(), 2);
    }
}
