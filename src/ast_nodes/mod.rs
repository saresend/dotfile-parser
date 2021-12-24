mod assignment;
mod edge;
mod node;
mod statement;
mod subgraph;

pub use assignment::Assignment;
pub use edge::Edge;
pub use node::Node;
pub use statement::Statement;
pub use subgraph::Subgraph;

type ID = String;

use crate::lex::{Peekable, PeekableLexer, Token};
use crate::parse::Constructable;

impl Constructable for ID {
    type Output = Self;
    fn from_lexer(
        mut token_stream: crate::lex::PeekableLexer,
    ) -> anyhow::Result<(Self::Output, crate::lex::PeekableLexer), anyhow::Error> {
        if let Some(Token::ID) = token_stream.next() {
            Ok((token_stream.slice().to_owned(), token_stream))
        } else {
            Err(anyhow::anyhow!("Expected type ID"))
        }
    }
}
