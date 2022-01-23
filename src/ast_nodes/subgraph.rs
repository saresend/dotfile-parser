use super::Statement;
use super::ID;

use crate::lex::{Peekable, Token};
use crate::parse::Constructable;

use super::edge::GraphDirection;

#[derive(Debug)]
pub struct Subgraph<T> {
    pub id: Option<ID>,
    pub statements: Vec<Statement<T>>,
}

impl<T: GraphDirection> Constructable for Subgraph<T> {
    type Output = Self;

    fn from_lexer(
        mut token_stream: crate::lex::PeekableLexer,
    ) -> anyhow::Result<(Self::Output, crate::lex::PeekableLexer), anyhow::Error> {
        token_stream.clear_filler();
        let mut id = None;
        if let Some(&Token::Subgraph) = token_stream.peek() {
            token_stream.next();
        }

        if let Some(&Token::ID) = token_stream.peek() {
            token_stream.next();
            id = Some(String::from(token_stream.slice()));
        }
        if let Some(Token::OpenParen) = token_stream.next() {
            let (statements, mut tok_stream) = Vec::<Statement<T>>::from_lexer(token_stream)?;
            tok_stream.clear_filler();
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

    use super::super::edge::Directed;
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

    #[test]
    fn test_subgraph_sample3_subset_test() {
        let test_str = "subgraph cluster_R {
            nd_3_l -> nd_3 -> nd_3_r [color=grey, arrowhead=none]
        }";
        let pb = PeekableLexer::from(test_str);
        let subgraph = Subgraph::<Directed>::from_lexer(pb).unwrap().0;
        assert_eq!(subgraph.id, Some(String::from("cluster_R")));
        assert_eq!(subgraph.statements.len(), 1);
    }

    #[test]
    fn test_subgraph_sample3_commaless_test() {
        let test_str = "subgraph cluster_R {
            nd_3_l -> nd_3 -> nd_3_r [color=grey arrowhead=none]
        }";
        let pb = PeekableLexer::from(test_str);
        let subgraph = Subgraph::<Directed>::from_lexer(pb).unwrap().0;
        assert_eq!(subgraph.id, Some(String::from("cluster_R")));
        assert_eq!(subgraph.statements.len(), 1);
    }

    #[test]
    fn test_subgraph_anonymous_undelimited_statement_test() {
        let test_str = "{rank=same nd_3_l nd_3 nd_3_r}";
        let pb = PeekableLexer::from(test_str);
        let subgraph = Subgraph::<Directed>::from_lexer(pb).unwrap().0;
        assert_eq!(subgraph.id, None);
        assert_eq!(subgraph.statements.len(), 4);
    }

    #[test]
    fn test_subgraph_statement_sanity4_test() {
        let test_str = "subgraph cluster_c2 {
                            label = \"Child two\";
                            te;
                        }";
        let pb = PeekableLexer::from(test_str);
        let subgraph = Subgraph::<Directed>::from_lexer(pb).unwrap().0;
        assert_eq!(subgraph.id, Some(String::from("cluster_c2")));
        assert_eq!(subgraph.statements.len(), 2);
        println!("{:#?}", subgraph);
    }
}
