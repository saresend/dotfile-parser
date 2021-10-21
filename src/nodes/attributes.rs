use super::common::IDNode;


#[derive(Clone, Debug)]
pub struct AttributeNode {
    lhs_id: IDNode,
    rhs_id: IDNode,
}

#[derive(Clone, Debug)]
pub struct AttributeListNode {
    attributes: Vec<AttributeNode>,
}

#[derive(Clone, Debug)]
pub enum AttributeStatementNode {
    Graph(Vec<AttributeListNode>),
    Node(Vec<AttributeListNode>),
    Edge(Vec<AttributeListNode>),
}
