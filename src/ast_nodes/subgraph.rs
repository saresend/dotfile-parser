use super::Statement;
use super::ID;

use crate::lex::{Peekable, Token};
use crate::parse::Constructable;

use super::edge::{Directed, Undirected};

#[derive(Debug)]
pub struct Subgraph<T> {
    pub id: Option<ID>,
    pub statements: Vec<Statement<T>>,
}

impl Constructable for Subgraph<Directed> {
    type Output = Self;

    fn from_lexer(
        mut token_stream: crate::lex::PeekableLexer,
    ) -> anyhow::Result<(Self::Output, crate::lex::PeekableLexer), anyhow::Error> {
        let mut id = None;
        if let Some(&Token::Subgraph) = token_stream.peek() {
            token_stream.next();
        }

        if let Some(&Token::ID) = token_stream.peek() {
            token_stream.next();
            id = Some(String::from(token_stream.slice()));
        }
        if let Some(Token::OpenParen) = token_stream.next() {
            let (statements, mut tok_stream) =
                Vec::<Statement<Directed>>::from_lexer(token_stream)?;
            if let Some(Token::CloseParen) = tok_stream.next() {
                Ok((Self { id, statements }, tok_stream))
            } else {
                Err(anyhow::anyhow!("Invalid closing paren for subgraph"))
            }
        } else {
            Err(anyhow::anyhow!("Invalid construction of subgraph"))
        }
    }
}

impl Constructable for Subgraph<Undirected> {
    type Output = Self;

    fn from_lexer(
        mut token_stream: crate::lex::PeekableLexer,
    ) -> anyhow::Result<(Self::Output, crate::lex::PeekableLexer), anyhow::Error> {
        let mut id = None;
        if let Some(&Token::Subgraph) = token_stream.peek() {
            token_stream.next();
        }

        if let Some(&Token::ID) = token_stream.peek() {
            token_stream.next();
            id = Some(String::from(token_stream.slice()));
        }
        if let Some(Token::OpenParen) = token_stream.next() {
            let (statements, mut tok_stream) =
                Vec::<Statement<Undirected>>::from_lexer(token_stream)?;
            if let Some(Token::CloseParen) = tok_stream.next() {
                Ok((Self { id, statements }, tok_stream))
            } else {
                Err(anyhow::anyhow!("Invalid closing paren for subgraph"))
            }
        } else {
            Err(anyhow::anyhow!("Invalid construction of subgraph"))
        }
    }
}

#[cfg(test)]
mod tests {

    use super::Directed;
    use super::Subgraph;
    use crate::lex::PeekableLexer;
    use crate::parse::Constructable;

    #[test]
    fn test_subgraph_sanity1_test() {
        let test_str = "test1 { A, B }";
        let pb = PeekableLexer::from(test_str);
        let subgraph = Subgraph::<Directed>::from_lexer(pb).unwrap().0;

        assert!(subgraph.id == Some(String::from("test1")));
        assert_eq!(subgraph.statements.len(), 2);
    }

    #[test]
    fn test_subgraph_sanity2_test() {
        let test_str = "{ A, B }";
        let pb = PeekableLexer::from(test_str);
        let subgraph = Subgraph::<Directed>::from_lexer(pb).unwrap().0;

        assert!(subgraph.id.is_none());
        assert_eq!(subgraph.statements.len(), 2);
    }

    #[test]
    fn test_subgraph_sanity3_test() {
        let test_str = "subgraph g { A, B }";

        let pb = PeekableLexer::from(test_str);
        let subgraph = Subgraph::<Directed>::from_lexer(pb).unwrap().0;

        assert_eq!(subgraph.id, Some(String::from("g")));
        assert_eq!(subgraph.statements.len(), 2);
    }
}
