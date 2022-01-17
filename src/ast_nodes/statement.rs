use crate::lex::Peekable;
use crate::parse::Constructable;

use super::assignment::*;
use super::edge::GraphDirection;
use super::{Edge, Node, Subgraph};

use crate::lex::Token;

#[derive(Debug)]
pub enum Statement<T> {
    Node(Box<Node>),
    Edge(Box<Edge<T>>),
    Attribute(Box<AttributeStatement>),
    Assignment(Box<Assignment>),
    Subgraph(Box<Subgraph<T>>),
}

impl<T: GraphDirection> Constructable for Statement<T> {
    type Output = Self;

    fn from_lexer(
        mut token_stream: crate::lex::PeekableLexer,
    ) -> anyhow::Result<(Self, crate::lex::PeekableLexer), anyhow::Error> {
        token_stream.clear_filler();
        if let Ok((assignment, tok_stream)) = Assignment::from_lexer(token_stream.clone()) {
            Ok((Self::Assignment(Box::new(assignment)), tok_stream))
        } else if let Ok((edge, tok_stream)) = Edge::<T>::from_lexer(token_stream.clone()) {
            Ok((Self::Edge(Box::new(edge)), tok_stream))
        } else if let Ok((node, tok_stream)) = Node::from_lexer(token_stream.clone()) {
            Ok((Self::Node(Box::new(node)), tok_stream))
        } else if let Ok((attribute, tok_stream)) =
            AttributeStatement::from_lexer(token_stream.clone())
        {
            Ok((Self::Attribute(Box::new(attribute)), tok_stream))
        } else if let Ok((subgraph, tok_stream)) = Subgraph::<T>::from_lexer(token_stream.clone()) {
            Ok((Self::Subgraph(Box::new(subgraph)), tok_stream))
        } else {
            Err(anyhow::anyhow!("Invalid statement"))
        }
    }
}

impl<T: GraphDirection> Constructable for Vec<Statement<T>> {
    type Output = Self;

    fn from_lexer(
        mut token_stream: crate::lex::PeekableLexer,
    ) -> anyhow::Result<(Self::Output, crate::lex::PeekableLexer), anyhow::Error> {
        let mut statements = vec![];
        while let Ok(statement) = Statement::<T>::from_lexer(token_stream.clone()) {
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
    use super::Directed;
    use super::Statement;
    use crate::lex::PeekableLexer;
    use crate::parse::Constructable;

    #[test]
    fn statement_enum_node_test() {
        let test_str = "A [color = blue, color = green]";
        let pbl = PeekableLexer::from(test_str);
        let result = Statement::<Directed>::from_lexer(pbl).unwrap().0;

        assert!(matches!(result, Statement::Node { .. }));
    }

    #[test]
    fn statement_enum_attributelist_test() {
        let test_str = "graph [color = blue, color = green]";
        let pbl = PeekableLexer::from(test_str);
        let result = Statement::<Directed>::from_lexer(pbl).unwrap().0;

        assert!(matches!(result, Statement::Attribute { .. }));
    }

    #[test]
    fn statement_enum_assignment_test() {
        let test_str = "color = blue";
        let pbl = PeekableLexer::from(test_str);
        let result = Statement::<Directed>::from_lexer(pbl).unwrap().0;
        assert!(matches!(result, Statement::Assignment { .. }));
    }
}
