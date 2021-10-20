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

// TODO: Should add compass pt support 
type Port = String;

type Ident = String; // using strings as Identifiers for now

#[derive(Clone, Debug)]
pub enum EdgeOP {
    Directed,
    Undirected,
}
#[derive(Clone, Debug)]
pub struct IDNode {
    id: Ident,  
}

#[derive(Clone, Debug)]
pub struct NodeStatementNode {
    id: NodeID,
    attributes: Vec<AttributeListNode>,
}

#[derive(Clone, Debug)]
pub struct NodeID {
    id: Ident,
    port: Option<Port>,
}

#[derive(Clone, Debug)]
pub enum EdgeRHSNode {
   Node((EdgeOP, NodeID, Box<Option<EdgeRHSNode>>)),
   Subgraph(SubgraphNode, Box<Option<EdgeRHSNode>>), 
}

#[derive(Clone, Debug)]
pub enum EdgeStatementNode {
    Node((NodeID, EdgeRHSNode, Vec<AttributeListNode>)),
    Subgraph((SubgraphNode, EdgeRHSNode, Vec<AttributeListNode>)),
}

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


#[derive(Clone, Debug)]
pub struct AssignmentStatementNode {
    lhs_id: IDNode, 
    rhs_id: IDNode,
}

#[derive(Clone, Debug)]
pub struct SubgraphNode {
    id: Option<IDNode>,
    statements: Vec<StatementNode>,
}
