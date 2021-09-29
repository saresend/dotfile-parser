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

    #[token("->")]
    DirectedEdge,

    #[token("--")]
    UndirectedEdge,

    #[error]
    Error,


}
