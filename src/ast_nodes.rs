#[derive(Clone, Debug)]
pub struct GraphNode {
    statements: Vec<StatementNode>,
}

#[derive(Clone, Debug)]
pub enum StatementNode {
    NodeStatement(NodeStatementNode),
    EdgeStatement(EdgeStatementNode),
    AttributeStatement(AttributeStatementNode),
    AssignmentStatement(AssignmentStatementNode),
    SubgraphStatement(SubgraphNode),
}

#[derive(Clone, Debug)]
pub struct NodeStatementNode {}

#[derive(Clone, Debug)]
pub struct EdgeStatementNode {}
#[derive(Clone, Debug)]

pub struct AttributeStatementNode {}
#[derive(Clone, Debug)]

pub struct AssignmentStatementNode {}

#[derive(Clone, Debug)]
pub struct SubgraphNode {}
