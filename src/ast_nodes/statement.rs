use crate::lex::Peekable;
use crate::parse::Constructable;

use super::assignment::*;
use super::edge::Directed;
use super::{Edge, Node, Subgraph};

use crate::lex::Token;

pub enum Statement<T> {
    Node(Box<Node>),
    Edge(Box<Edge<T>>),
    Attribute(Box<AttributeList>),
    Assignment(Box<Assignment>),
    Subgraph(Box<Subgraph<T>>),
}

impl Constructable for Statement<Directed> {
    type Output = Self;

    fn from_lexer(
        token_stream: crate::lex::PeekableLexer,
    ) -> anyhow::Result<(Self, crate::lex::PeekableLexer), anyhow::Error> {
        if let Ok((assignment, tok_stream)) = Assignment::from_lexer(token_stream.clone()) {
            Ok((Self::Assignment(Box::new(assignment)), tok_stream))
        } else if let Ok((node, tok_stream)) = Node::from_lexer(token_stream.clone()) {
            Ok((Self::Node(Box::new(node)), tok_stream))
        } else if let Ok((attribute, tok_stream)) = AttributeList::from_lexer(token_stream.clone())
        {
            Ok((Self::Attribute(Box::new(attribute)), tok_stream))
        } else if let Ok((subgraph, tok_stream)) =
            Subgraph::<Directed>::from_lexer(token_stream.clone())
        {
            Ok((Self::Subgraph(Box::new(subgraph)), tok_stream))
        } else if let Ok((edge, tok_stream)) = Edge::<Directed>::from_lexer(token_stream.clone()) {
            Ok((Self::Edge(Box::new(edge)), tok_stream))
        } else {
            Err(anyhow::anyhow!("Invalid statement"))
        }
    }
}

impl Constructable for Vec<Statement<Directed>> {
    type Output = Self;

    fn from_lexer(
        mut token_stream: crate::lex::PeekableLexer,
    ) -> anyhow::Result<(Self::Output, crate::lex::PeekableLexer), anyhow::Error> {
        let mut statements = vec![];
        while let Ok(statement) = Statement::from_lexer(token_stream.clone()) {
            token_stream = statement.1;
            statements.push(statement.0);
            match token_stream.peek() {
                Some(&Token::SemiColon) | Some(&Token::Comma) => {
                    token_stream.next();
                }
                _ => {} // Intentional no-op
            };
        }
        Ok((statements, token_stream))
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
