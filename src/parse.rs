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
    use std::fs::read_to_string;

    use super::Constructable;
    use super::ParseOR;
    use crate::ast_nodes::{Assignment, Node};
    use crate::ast_nodes::{Directed, Graph};
    use crate::lex::PeekableLexer;
    use std::io::Write;

    fn test_for_file(f: &str) -> Graph<Directed> {
        let v = read_to_string(f).unwrap();
        let pb = PeekableLexer::from(&v);
        Graph::<Directed>::from_lexer(pb).unwrap().0
    }

    fn updateable_test(sample_loc: &str, ref_loc: &str) {
        if std::env::var("UPDATE_TESTS").is_ok() {
            println!("Updating test");
            let g = test_for_file(sample_loc);
            let mut f = std::fs::File::create(ref_loc).unwrap();
            f.write_all(format!("{:#?}", g).as_bytes()).unwrap();
        } else {
            let g = test_for_file(sample_loc);
            let reference = std::fs::read_to_string(ref_loc).unwrap();
            assert_eq!(format!("{:#?}", g), reference);
        }
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
        updateable_test("samples/basic1.dot", "samples/reference/basic1.ref");
    }

    #[test]
    fn test_ast_build_basic2_test() {
        updateable_test("samples/basic2.dot", "samples/reference/basic2.ref");
    }

    #[test]
    fn test_ast_build_basic3_test() {
        updateable_test("samples/basic3.dot", "samples/reference/basic3.ref");
    }

    #[test]
    fn test_ast_build_basic4_test() {
        updateable_test("samples/basic4.dot", "samples/reference/basic4.ref");
    }

    #[test]
    fn test_ast_build_datastruct_test() {
        updateable_test("samples/datastruct.dot", "samples/reference/datastruct.ref");
    }
}
