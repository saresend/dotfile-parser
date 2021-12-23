use super::assignment::AttributeList;
use super::node::Node;
use super::Subgraph;
use super::ID;
use std::marker::PhantomData;

use crate::lex::{Peekable, Token};

use crate::parse::{Constructable, ParseOR};

struct Directed;
struct Undirected;

enum EdgeLHS {
    Node(Node),
    Subgraph(Subgraph),
}

pub struct Edge<T>  {
    lhs: EdgeLHS,
    rhs: Box<Edge<T>>,
    ty: PhantomData<T>,
    attr_list: AttributeList,

}
