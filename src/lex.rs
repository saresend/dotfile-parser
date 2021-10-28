use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
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

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]+")]
    ID,

    #[token("strict")]
    Strict,

    #[token("subgraph")]
    Subgraph,
    #[token("graph")]
    Graph,
    #[token("digraph")]
    Digraph,
    #[token("node")]
    Node,
    #[token("edge")]
    Edge,
    #[token("->")]
    DirectedEdge,
    #[token("--")]
    UndirectedEdge,
    #[token("{")]
    OpenParen,
    #[token("}")]
    CloseParen,

    #[token(",")]
    Comma,
    #[token(";")]
    SemiColon,
    #[token(":")]
    Colon,

    #[token("\"")]
    Quotation,

    #[error]
    Error,
}

pub trait Peekable {
    type Item;
    fn peek(&mut self) -> Option<&Self::Item>;
}

/// A lexing wrapper that supports the method
/// .peek()
pub struct PeekableLexer<'a> {
    inner_lexer: logos::Lexer<'a, Token>,
    peeked_token: Option<Token>,
}

impl<'a> std::iter::Iterator for PeekableLexer<'a> {
    type Item = Token;

    /// This will actually consume the next token if we don't have an existing token that
    /// has earlier been peeked, otherwise it will return the peeked token
    fn next(&mut self) -> Option<Token> {
        if let Some(inner_tok) = self.peeked_token.take() {
            Some(inner_tok)
        } else {
            self.inner_lexer.next()
        }
    }
}

impl<'a> Peekable for PeekableLexer<'a> {
    type Item = Token;
    fn peek(&mut self) -> Option<&Token> {
        if self.peeked_token.is_none() {
            self.peeked_token = self.inner_lexer.next();
        }
        self.peeked_token.as_ref()
    }
}

impl<'a> PeekableLexer<'a> {
    /// Constructs a new instance of the PeekableLexer
    pub fn new(inner_lexer: logos::Lexer<'a, Token>) -> Self {
        Self {
            inner_lexer,
            peeked_token: None,
        }
    }
}
