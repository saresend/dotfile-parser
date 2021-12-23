use super::Statement;
use super::ID;

pub struct Subgraph {
    id: Option<ID>,
    statements: Vec<Statement>,
}
