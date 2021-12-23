use super::ID;
use crate::parse::Constructable;
use std::convert::TryFrom;

use crate::lex::{Peekable, PeekableLexer, Token};

/// This is the primary node capable of parsing
/// constructs of the form 'ID' = 'ID'
#[derive(Debug, Clone, PartialEq)]
pub struct Assignment {
    lhs: ID,
    rhs: ID,
}

impl Assignment {
    pub fn new(lhs: &str, rhs: &str) -> Self {
        Assignment {
            lhs: lhs.to_owned(),
            rhs: rhs.to_owned(),
        }
    }
}

pub type AttributeList = Vec<AssignmentGroup>;

pub type AssignmentGroup = Vec<Assignment>;

impl Constructable for Assignment {
    type Output = Self;

    fn from_lexer(
        mut lexer: PeekableLexer<'_>,
    ) -> Result<(Self::Output, PeekableLexer), anyhow::Error> {
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
    type Output = Self;

    fn from_lexer(
        mut token_stream: PeekableLexer<'_>,
    ) -> Result<(Self::Output, PeekableLexer), anyhow::Error> {
        let mut result = vec![];
        while let Ok((assignment, stream)) = Assignment::from_lexer(token_stream.clone()) {
            result.push(assignment);
            token_stream = stream;
            match token_stream.peek() {
                Some(Token::Comma) | Some(Token::SemiColon) => {
                    token_stream.next();
                }
                _ => {
                    return Ok((result, token_stream));
                }
            }
        }
        Ok((result, token_stream))
    }
}

impl Constructable for AttributeList {
    type Output = Self;

    fn from_lexer(
        mut token_stream: PeekableLexer<'_>,
    ) -> Result<(Self::Output, PeekableLexer), anyhow::Error> {
        let mut result = vec![];
        if token_stream.peek() != Some(&Token::OpenBracket) {
            return Err(anyhow::anyhow!("Invalid token to construct attributeList"));
        }
        while let Some(Token::OpenBracket) = token_stream.next() {
            let agroup = AssignmentGroup::from_lexer(token_stream.clone())?;
            result.push(agroup.0);
            token_stream = agroup.1;
            match token_stream.next() {
                Some(Token::CloseBracket) => {}
                _ => return Err(anyhow::anyhow!("Mismatched Tokens")),
            }
        }
        Ok((result, token_stream))
    }
}

#[cfg(test)]
mod tests {
    use super::{Assignment, AssignmentGroup, AttributeList};
    use crate::lex::PeekableLexer;
    use crate::parse::Constructable;

    #[test]
    fn assignment_sanity_test() {
        let test_str = "color = red";
        let plexer = PeekableLexer::from(test_str);
        let assignment = Assignment::from_lexer(plexer).unwrap().0;
        assert_eq!(assignment.lhs, String::from("color"));
        assert_eq!(assignment.rhs, String::from("red"));
    }

    #[test]
    fn assignment_correct_rejection_test() {
        let test_str = "color = {";
        let plexer = PeekableLexer::from(test_str);
        let assignment = Assignment::from_lexer(plexer);
        assert!(assignment.is_err());
    }

    #[test]
    fn assignment_vector_comma_sanity_test() {
        let test_str = "color = red, width = hello";
        let plexer = PeekableLexer::from(test_str);
        let results: Vec<Assignment> = Vec::<Assignment>::from_lexer(plexer).unwrap().0;
        assert_eq!(results[0].lhs, String::from("color"));
        assert_eq!(results[0].rhs, String::from("red"));
        assert_eq!(
            results[1],
            Assignment {
                lhs: String::from("width"),
                rhs: String::from("hello")
            }
        );
    }

    #[test]
    fn assignment_attribute_list_sanity_test() {
        let test_str = "[ color = red ][ color = red ]";
        let valid = vec![Assignment {
            lhs: String::from("color"),
            rhs: String::from("red"),
        }];
        let plexer = PeekableLexer::from(test_str);
        let result: AttributeList = AttributeList::from_lexer(plexer).unwrap().0;
        assert_eq!(result[0], valid);
        assert_eq!(result[1], valid);
    }

    #[test]
    fn assignment_attribute_list_sanity2_test() {
        let test_str = "[ color = red, color = red ][ color = red ]";
        let valid = vec![Assignment {
            lhs: String::from("color"),
            rhs: String::from("red"),
        }];
        let plexer = PeekableLexer::from(test_str);
        let result: AttributeList = AttributeList::from_lexer(plexer).unwrap().0;

        assert_eq!(result[0][0], valid[0]);
        assert_eq!(result[0][1], valid[0]);
        assert_eq!(result[1], valid);
    }

    #[test]
    fn assignment_attribute_list_failure1_test() {
        let test_str = "[ color = red, color = red ][ color = { ]";
        let plexer = PeekableLexer::from(test_str);
        let result = Vec::<AssignmentGroup>::from_lexer(plexer);
        assert!(result.is_err());
    }
}
