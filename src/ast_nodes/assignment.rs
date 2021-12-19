use super::ID;
use std::convert::TryFrom;

use crate::lex::{Token, PeekableLexer, Peekable};

/// This is the primary node capable of parsing 
/// constructs of the form 'ID' = 'ID'
#[derive(Debug, PartialEq)]
pub struct Assignment {
    lhs: ID,
    rhs: ID,
}

impl TryFrom<&mut PeekableLexer<'_>> for Assignment {
    type Error = anyhow::Error;

    fn try_from(mut lexer: &mut PeekableLexer<'_>) -> Result<Self, Self::Error> {
        if let Some(Token::ID) = lexer.next() {
            let lhs = String::from(lexer.slice());
            if let Some(Token::Equals) = lexer.next() {
                if let Some(Token::ID) = lexer.next() {
                    let rhs = String::from(lexer.slice());
                    return Ok(Self { lhs, rhs });
                }
            }
        }
        Err(anyhow::anyhow!("Mismatched Tokens"))
    }
}


impl TryFrom<&mut PeekableLexer<'_>> for Vec<Assignment> {
    type Error = anyhow::Error;

    fn try_from(token_stream: &mut PeekableLexer<'_>) -> Result<Self, Self::Error> {
        let mut c1 = token_stream.clone();
        let mut result = vec![];
        while let Ok(assignment) = Assignment::try_from(&mut c1) {
            result.push(assignment);
            match c1.peek() {
                Some(Token::Comma) | Some(Token::SemiColon) => {
                    c1.next();
                },
                _ => {
                    *token_stream = c1;
                    return Ok(result);
                },
            }
        }
        *token_stream = c1;
        Ok(result)
    }

}

#[cfg(test)]
mod tests {
    use super::Assignment;
    use crate::lex::PeekableLexer;
    use std::convert::TryFrom;

    #[test]
    fn assignment_sanity_test() {
        let test_str = "color = red";
        let mut plexer = PeekableLexer::from(test_str); 
        let assignment = Assignment::try_from(&mut plexer).unwrap();
        assert_eq!(assignment.lhs, String::from("color"));
        assert_eq!(assignment.rhs, String::from("red"));
    }

    #[test]
    fn assignment_correct_rejection_test() {
        let test_str = "color = {";
        let mut plexer = PeekableLexer::from(test_str);
        let assignment = Assignment::try_from(&mut plexer);
        assert!(assignment.is_err());

    }

    #[test]
    fn assignment_vector_comma_sanity_test() {
        let test_str = "color = red, width = hello";
        let mut plexer = PeekableLexer::from(test_str);
        let results = Vec::try_from(&mut plexer).unwrap();
        assert_eq!(results[0].lhs, String::from("color"));
        assert_eq!(results[0].rhs, String::from("red"));
        assert_eq!(results[1], Assignment { lhs: String::from("width"), rhs: String::from("hello") });



    }
}
