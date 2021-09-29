use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
enum Token {


    #[token("n")]
    CompassPtNorth, 
    #[token("ne")]
    CompassPtNorthEast,
    #[token("e")]
    CompassPtEast,
    #[token("se")]
    CompassPtSouthEast,
    #[token("s")]
    CompassPtSouth, 
    #[token("sw")]
    CompassPtSouthWest, 
    #[token("w")]
    CompassPtWest, 
    #[token("nw")]
    CompassPtNorthWest,

    #[token("subgraph")]
    Subgraph, 
    #[token("graph")]
    Graph, 
    #[token("digraph")]
    Digraph,
    #[token("strict")]
    #[token("node")]
    Node, 
    #[token("edge")]
    Edge,
    #[token("->")]
    DirectedEdge,
    #[token("--")]
    UndirectedEdge,
    #[error]
    Error,


}
