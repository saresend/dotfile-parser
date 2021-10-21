// TODO: Should add compass pt support
type Port = String;

type Ident = String; // using strings as Identifiers for now

#[derive(Clone, Debug)]
pub struct IDNode {
    id: Ident,
}

#[derive(Clone, Debug)]
pub struct NodeID {
    id: Ident,
    port: Option<Port>,
}

#[derive(Clone, Debug)]
pub struct AssignmentStatementNode {
    lhs_id: IDNode,
    rhs_id: IDNode,
}
