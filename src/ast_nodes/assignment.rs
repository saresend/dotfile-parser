use super::ID;
use std::convert::TryFrom;

use crate::lex::{Token, PeekableLexer, Peekable};

/// This is the primary node capable of parsing 
/// constructs of the form 'ID' = 'ID'
pub struct Assignment {
    lhs: ID,
    rhs: ID,
}



impl TryFrom<PeekableLexer<'_>> for Assignment {
    type Error = anyhow::Error;

    fn try_from(mut lexer: PeekableLexer<'_>) -> Result<Self, Self::Error> {
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


#[cfg(test)]
mod tests {
    use super::Assignment;
    use crate::lex::PeekableLexer;
    use std::convert::TryFrom;

    #[test]
    fn assignment_sanity_test() {
        let test_str = "color = red";
        let plexer = PeekableLexer::from(test_str); 
        let assignment = Assignment::try_from(plexer).unwrap();
        assert_eq!(assignment.lhs, String::from("color"));
        assert_eq!(assignment.rhs, String::from("red"));
    }

    #[test]
    fn assignment_correct_rejection_test() {
        let test_str = "color = {";
        let plexer = PeekableLexer::from(test_str);
        let assignment = Assignment::try_from(plexer);
        assert!(assignment.is_err());

    }
}
