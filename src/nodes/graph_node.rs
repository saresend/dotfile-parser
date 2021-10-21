use super::statement_node::StatementNode;
use super::common::IDNode;
use crate::lex::Token;

#[derive(Clone, Debug)]
pub struct GraphNode {
    statements: Vec<StatementNode>,
}

impl GraphNode {
    pub fn parse_from_tks<'a>(
        token_stream: &mut impl Iterator<Item = Token>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        while let Some(c_tok) = token_stream.next() {
            match c_tok {
                Token::OpenParen => {
                         
                },
                _ => {},
            }
        }
        todo!()
    }
}


#[derive(Clone, Debug)]
pub struct SubgraphNode {
    id: Option<IDNode>,
    statements: Vec<StatementNode>,
}
