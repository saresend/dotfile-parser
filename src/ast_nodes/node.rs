use crate::parse::Constructable;

use super::{assignment::AttributeList, ID};
use crate::lex::{Peekable, Token};

/// The main ASTNode type that represents
/// any sort of node statement that configures attributes for a node
/// example: 'A [color = red][length = long]'
pub struct Node {
    id: ID,
    attribute_list: AttributeList,
}

impl Constructable for Node {
    fn from_lexer(
        mut token_stream: crate::lex::PeekableLexer,
    ) -> anyhow::Result<(Self, crate::lex::PeekableLexer), anyhow::Error> {
        if let Some(Token::ID) = token_stream.next() {
            let node_id = token_stream.slice().to_owned();
            let agroup = AttributeList::from_lexer(token_stream)?;
            return Ok((
                Self {
                    id: node_id,
                    attribute_list: agroup.0,
                },
                agroup.1,
            ));
        }
        Err(anyhow::anyhow!("Invalid Node; can't find ID"))
    }
}

#[cfg(test)]
mod tests {
    use crate::ast_nodes::assignment::*;
    use crate::lex::PeekableLexer;
    use crate::parse::Constructable;

    use super::Node;

    #[test]
    fn node_statement_sanity1_test() {
        let test_str = "A [color = blue, height = s10][length = long]";
        let lexer = PeekableLexer::from(test_str);
        let node = Node::from_lexer(lexer).unwrap().0;
        let asgng_1 = vec![
            Assignment::new("color", "blue"),
            Assignment::new("height", "s10"),
        ];
        let asgng_2 = vec![Assignment::new("length", "long")];
        assert_eq!(node.attribute_list[0], asgng_1);
    }

    #[test]
    fn node_statement_sanity2_test() {
        let test_str = "color = blue";
        let lexer = PeekableLexer::from(test_str);
        let node = Node::from_lexer(lexer);
        assert!(node.is_err());
    }
}
