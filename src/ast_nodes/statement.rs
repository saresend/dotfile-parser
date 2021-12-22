use crate::parse::Constructable;

use super::node::Node;
use super::assignment::*;

pub enum Statement {
    Node(Box<Node>),
    Attribute(Box<AttributeList>),
    Assignment(Box<Assignment>),
}


impl Constructable for Statement {

    fn from_lexer(token_stream: crate::lex::PeekableLexer) -> anyhow::Result<(Self, crate::lex::PeekableLexer), anyhow::Error> {
        if let Ok((node, tok_stream)) = Node::from_lexer(token_stream.clone()) {
            Ok((Self::Node(Box::new(node)), tok_stream))
        } else if let Ok((attribute, tok_stream)) = AttributeList::from_lexer(token_stream.clone()) {
            Ok((Self::Attribute(Box::new(attribute)), tok_stream))
        } else if let Ok((assignment, tok_stream)) = Assignment::from_lexer(token_stream.clone()) {
            Ok((Self::Assignment(Box::new(assignment)), tok_stream))
        } else {
            todo!()
        }
    }

}



#[cfg(test)] 
mod tests {
    use super::Statement;
    use crate::lex::PeekableLexer;
    use crate::parse::Constructable;

    #[test]
    fn statement_enum_node_test() {
        let test_str = "A [color = blue, color = green]"; 
        let pbl = PeekableLexer::from(test_str);
        let result = Statement::from_lexer(pbl).unwrap().0;

        assert!(matches!(result, Statement::Node { .. }));

    }

    #[test]
    fn statement_enum_attributelist_test() {
        let test_str = "[color = blue, color = green]"; 
        let pbl = PeekableLexer::from(test_str);
        let result = Statement::from_lexer(pbl).unwrap().0;

        assert!(matches!(result, Statement::Attribute { .. }));
    }

    #[test]
    fn statement_enum_assignment_test() {
        let test_str = "color = blue"; 
        let pbl = PeekableLexer::from(test_str);
        let result = Statement::from_lexer(pbl).unwrap().0;

        assert!(matches!(result, Statement::Assignment { .. }));
    }




}
