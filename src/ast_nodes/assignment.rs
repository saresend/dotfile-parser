use super::ID;
use std::convert::TryFrom;
use crate::parse::Constructable;

use crate::lex::{Token, PeekableLexer, Peekable};

/// This is the primary node capable of parsing 
/// constructs of the form 'ID' = 'ID'
#[derive(Debug, PartialEq)]
pub struct Assignment {
    lhs: ID,
    rhs: ID,
}

pub type AttributeList = Vec<AssignmentGroup>;

pub type AssignmentGroup = Vec<Assignment>;

impl Constructable for Assignment {
    fn from_lexer(mut lexer: PeekableLexer<'_>) -> Result<(Self, PeekableLexer), anyhow::Error> {
        if let Some(Token::ID) = lexer.next() {
            let lhs = String::from(lexer.slice());
            if let Some(Token::Equals) = lexer.next() {
                if let Some(Token::ID) = lexer.next() {
                    let rhs = String::from(lexer.slice());
                    return Ok((Self { lhs, rhs }, lexer));
                }
            }
        }
        Err(anyhow::anyhow!("Mismatched Tokens"))
    }
}


impl Constructable for AssignmentGroup {

    fn from_lexer(mut token_stream: PeekableLexer<'_>) -> Result<(Self, PeekableLexer), anyhow::Error> {
        let mut result = vec![];
        while let Ok((assignment, stream)) = Assignment::from_lexer(token_stream.clone()) {
            result.push(assignment);
            token_stream = stream;
            match token_stream.peek() {
                Some(Token::Comma) | Some(Token::SemiColon) => {
                    token_stream.next();
                },
                _ => {
                    return Ok((result, token_stream));
                },
            }
        }
        Ok((result, token_stream))
    }

}

impl Constructable for AttributeList {
    fn from_lexer(mut token_stream: PeekableLexer<'_>) -> Result<(Self, PeekableLexer), anyhow::Error> {
        let mut result = vec![];
        while let Some(Token::OpenBracket) = token_stream.next() {
            let agroup = AssignmentGroup::from_lexer(token_stream.clone())?;
            result.push(agroup.0);
            token_stream = agroup.1;
            match token_stream.next() {
                Some(Token::CloseBracket) => {},
                _ => return Err(anyhow::anyhow!("Mismatched Tokens")),
            }
        }
        Ok((result, token_stream))
    }
}

#[cfg(test)]
mod tests {
    use super::{AttributeList, Assignment};
    use crate::lex::PeekableLexer;
    use std::convert::TryFrom;
    use crate::parse::Constructable;

    #[test]
    fn assignment_sanity_test() {
        let test_str = "color = red";
        let mut plexer = PeekableLexer::from(test_str); 
        let assignment = Assignment::from_lexer(plexer).unwrap().0;
        assert_eq!(assignment.lhs, String::from("color"));
        assert_eq!(assignment.rhs, String::from("red"));
    }

    #[test]
    fn assignment_correct_rejection_test() {
        let test_str = "color = {";
        let mut plexer = PeekableLexer::from(test_str);
        let assignment = Assignment::from_lexer(plexer);
        assert!(assignment.is_err());

    }

    #[test]
    fn assignment_vector_comma_sanity_test() {
        let test_str = "color = red, width = hello";
        let mut plexer = PeekableLexer::from(test_str);
        let results: Vec<Assignment> = Vec::from_lexer(plexer).unwrap().0;
        assert_eq!(results[0].lhs, String::from("color"));
        assert_eq!(results[0].rhs, String::from("red"));
        assert_eq!(results[1], Assignment { lhs: String::from("width"), rhs: String::from("hello") });
    }

    #[test]
    fn assignment_attribute_list_sanity_test() {
        let test_str = "[ color = red ][ color = red ]";
        let valid = vec![Assignment { lhs: String::from("color"), rhs : String::from("red") }];
        let mut plexer = PeekableLexer::from(test_str);
        let result: AttributeList = Vec::from_lexer(plexer).unwrap().0;
        assert_eq!(result[0], valid);
        assert_eq!(result[1], valid);
    }
}
