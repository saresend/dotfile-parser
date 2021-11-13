use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
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
use logos::Span;

pub trait Peekable<'a> {
    type Item;
    fn peek(&mut self) -> Option<&Self::Item>;
    fn span(&self) -> Span;
    fn slice(&self) -> &'a str;
}

/// A lexing wrapper that supports the method
/// .peek()
#[derive(Clone)]
pub struct PeekableLexer<'a> {
    inner_lexer: logos::Lexer<'a, Token>,
    peeked_token: Option<Token>,
    curr_span: Span,
    curr_slice: &'a str,
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

impl<'a> Peekable<'a> for PeekableLexer<'a> {
    type Item = Token;

    fn peek(&mut self) -> Option<&Token> {
        if self.peeked_token.is_none() {
            self.curr_span = self.inner_lexer.span();
            self.peeked_token = self.inner_lexer.next();
        }
        self.peeked_token.as_ref()
    }

    fn span(&self) -> Span {
        if self.peeked_token.is_none() {
            self.inner_lexer.span()
        } else {
            self.curr_span.clone()
        }
    }

    fn slice(&self) -> &'a str {
        if self.peeked_token.is_none() {
            self.inner_lexer.slice()
        } else {
            self.curr_slice
        }
    }
}

impl<'a> PeekableLexer<'a> {
    /// Constructs a new instance of the PeekableLexer
    pub fn new(inner_lexer: logos::Lexer<'a, Token>) -> Self {
        let curr_span = inner_lexer.span().clone();
        let curr_slice = inner_lexer.slice();
        Self {
            inner_lexer,
            peeked_token: None,
            curr_span,
            curr_slice,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn lexer_slice_indexing_1_test() {
        let solution = vec!["big", "kahuna", "electric", "boogaloo"];
        let test_text: String = solution.iter().map(|x| x.chars()).flatten().collect();
        let mut lexer_to_test = PeekableLexer::new(Token::lexer(&test_text));

        for sol in solution {
            let _val = lexer_to_test.next();
            let _ = lexer_to_test.peek();
            assert_eq!(lexer_to_test.slice(), sol);
        }
    }
}
