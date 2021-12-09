use crate::parse::DotParseable;
use crate::lex::{Token, Peekable};
// TODO: Should add compass pt support
type Port = String;

type Ident = String; // using strings as Identifiers for now

#[derive(Clone, Debug)]
pub struct IDNode {
    id: Ident,
}

#[derive(Clone, Debug)]
pub struct NodeID {
    pub id: Ident,
    pub port: Option<Port>,
}

#[derive(Clone, Debug)]
pub struct AssignmentStatementNode {
    lhs_id: IDNode,
    rhs_id: IDNode,
}


impl DotParseable for AssignmentStatementNode {
    fn from_lexer<'a>(
        token_stream: &mut (impl Iterator<Item = Token> + Peekable<'a>),
    ) -> Result<Self, anyhow::Error> {
        todo!()
    }

}
