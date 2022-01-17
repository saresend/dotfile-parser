use super::lex::PeekableLexer;
use anyhow::Result;


pub(crate) trait Constructable: Sized {
    type Output;
    fn from_lexer(
        token_stream: PeekableLexer,
    ) -> Result<(Self::Output, PeekableLexer), anyhow::Error>;
}

pub(crate) struct ParseOR<T: Constructable, V: Constructable> {
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
    use crate::ast_nodes::{Directed, Graph};
    use crate::lex::PeekableLexer;

    fn test_for_file(file_path: &str) -> Graph<Directed> {
        let test_str = ::std::fs::read_to_string(file_path).unwrap();
        let pb = PeekableLexer::from(&test_str);
        Graph::<Directed>::from_lexer(pb).unwrap().0
    }

    #[test]
    fn or_op_sanity_test1() {
        let test_str = "color = green";
        let pb = PeekableLexer::from(test_str);
        let result: ParseOR<Assignment, Node> = ParseOR::from_lexer(pb).unwrap().0;
        assert!(result.t_val.is_some());
        assert_eq!(result.t_val.unwrap(), Assignment::new("color", "green"));
    }

    #[test]
    fn test_ast_build_sanity1() {
        let test_str = "digraph G { subgraph t1 { A [color = green] } }";
        let pb = PeekableLexer::from(test_str);
        let g = Graph::<Directed>::from_lexer(pb).unwrap().0;
        assert_eq!(g.id, String::from("G"));
        assert_eq!(g.statements.len(), 1);
    }

    #[test]
    fn test_ast_build_basic1_test() {
        let g = test_for_file("samples/basic1.dot");
        let reference = std::fs::read_to_string("samples/reference/basic1.ref").unwrap();
        assert_eq!(reference, format!("{:?}\n", g));
    }

    #[test]
    fn test_ast_build_basic2_test() {
        let g = test_for_file("samples/basic2.dot");
        let reference = std::fs::read_to_string("samples/reference/basic2.ref").unwrap();
        assert_eq!(reference, format!("{:#?}\n", g));
    }

    #[test]
    fn test_ast_build_basic3_test() {
        let g = test_for_file("samples/basic3.dot");
        let reference = std::fs::read_to_string("samples/reference/basic3.ref").unwrap();
        assert_eq!(reference, format!("{:#?}\n", g));
    }
}
