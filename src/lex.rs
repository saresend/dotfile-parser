use logos::Logos;

/// A Token represents all terminals supported by the graphviz dot format
///
/// For more info on the tokens, see
/// the graphviz language spec here: https://graphviz.org/doc/info/lang.html
#[derive(Logos, Debug, PartialEq, Clone)]
pub(crate) enum Token<'a> {
    #[regex(r##"("([^"]|\\")*"|[a-zA-Z0-9_]+|-?(\.[0-9]+|[0-9]+(\.[0-9]*)?))"##)]
    ID(&'a str),

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

    #[token("[")]
    OpenBracket,

    #[token("]")]
    CloseBracket,

    #[token("=")]
    Equals,

    #[token(",")]
    Comma,

    #[token(";")]
    SemiColon,

    #[token("\n")]
    #[regex(r"//[^\n]*\n")] // line comment is like a NewLine
    NewLine,

    #[token(":")]
    Colon,

    #[token("\"")]
    Quotation,

    #[error]
    #[regex(r"[ \t\f]", logos::skip)]
    // actual block comment regexp picked from lalrpop documentation, seems to work :)
    #[regex(r##"/\*[^*]*\*+(?:[^/*][^*]*\*+)*/"##, logos::skip)]
    Error,
}
use logos::Span;

/// Remove the quotes from "quoted identifiers"
fn unquote_quoted_id<'a>(lex: &mut logos::Lexer<'a, Token<'a>>) -> Option<&'a str> {
    let slice = lex.slice();
    assert!(slice.len() >= 2);
    assert!(slice[0..1] == *"\"");
    assert!(slice[slice.len() - 1..] == *"\"");
    Some(&slice[1..slice.len() - 1])
}

/// The Peekable Trait extends the underlying
/// token iterator to support basic lookahead
/// it also provides
pub trait Peekable<'a> {
    type Item;
    fn peek(&mut self) -> Option<&Self::Item>;
    fn span(&self) -> Span;
    fn slice(&self) -> &'a str;
}

/// A lexer wrapper that supports the method
/// .peek() in addition to the standard set
/// of lexing operations
#[derive(Clone)]
pub(crate) struct PeekableLexer<'a> {
    inner_lexer: logos::Lexer<'a, Token<'a>>,
    peeked_token: Option<Token<'a>>,
    curr_span: Span,
    curr_slice: &'a str,
}

impl<'a> std::fmt::Debug for PeekableLexer<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut v = self.clone();
        while v.next().is_some() {
            write!(f, "{} ", v.slice())?;
        }
        write!(f, "\n")
    }
}

impl<'a> std::iter::Iterator for PeekableLexer<'a> {
    type Item = Token<'a>;

    /// This will consume the next token if we don't have an existing token that
    /// has earlier been peeked, otherwise it will return the peeked token
    fn next(&mut self) -> Option<Token<'a>> {
        if let Some(inner_tok) = self.peeked_token.take() {
            self.update_splice();
            Some(inner_tok)
        } else {
            let token = self.inner_lexer.next();
            self.update_splice();
            token
        }
    }
}

impl<'a> Peekable<'a> for PeekableLexer<'a> {
    type Item = Token<'a>;

    fn peek(&mut self) -> Option<&Token<'a>> {
        if self.peeked_token.is_none() {
            self.update_splice();
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
        self.curr_slice.as_ref()
    }
}

impl<'a> PeekableLexer<'a> {
    /// Creates a new lexer from a raw string
    pub fn from(ref_str: &'a str) -> Self {
        let inner_lexer = logos::Lexer::new(ref_str);
        Self::from_lexer(inner_lexer)
    }

    /// Constructs a new instance of the PeekableLexer
    /// from an existing underlying lexer
    fn from_lexer(inner_lexer: logos::Lexer<'a, Token<'a>>) -> Self {
        let curr_span = inner_lexer.span().clone();
        let curr_slice = inner_lexer.slice();
        Self {
            inner_lexer,
            peeked_token: None,
            curr_span,
            curr_slice,
        }
    }

    /// A utility method used to clear out lines that only used to delimit constructions
    pub(crate) fn clear_filler(&mut self) {
        while self.peek() == Some(&Token::NewLine) || self.peek() == Some(&Token::SemiColon) {
            self.next();
        }
    }

    fn update_splice(&mut self) {
        self.curr_span = self.inner_lexer.span();
        self.curr_slice = self.inner_lexer.slice();
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn lexer_test_lex_basic_dotfile() {
        let test_str = "strict graph {
                        a -- b
                        b -- a [color=blue]
                        }
        ";
        let mut lexer_sut = PeekableLexer::from(test_str);
        assert_eq!(lexer_sut.next(), Some(Token::Strict));
        assert_eq!(lexer_sut.next(), Some(Token::Graph));
        assert_eq!(lexer_sut.next(), Some(Token::OpenParen));
        assert_eq!(lexer_sut.next(), Some(Token::NewLine));
        assert_eq!(lexer_sut.next(), Some(Token::ID("a")));
        assert_eq!(lexer_sut.next(), Some(Token::UndirectedEdge));
        assert_eq!(lexer_sut.next(), Some(Token::ID("b")));
        assert_eq!(lexer_sut.next(), Some(Token::NewLine));
        assert_eq!(lexer_sut.next(), Some(Token::ID("b")));
        assert_eq!(lexer_sut.next(), Some(Token::UndirectedEdge));
        assert_eq!(lexer_sut.next(), Some(Token::ID("a")));

        assert_eq!(lexer_sut.next(), Some(Token::OpenBracket));
        assert_eq!(lexer_sut.next(), Some(Token::ID("color")));
        assert_eq!(lexer_sut.next(), Some(Token::Equals));
        assert_eq!(lexer_sut.next(), Some(Token::ID("blue")));
        assert_eq!(lexer_sut.next(), Some(Token::CloseBracket));
        assert_eq!(lexer_sut.next(), Some(Token::NewLine));
    }

    #[test]
    fn token_test_for_id_regex() {
        let test_str = "\"___ooogabooga:asdf\"";
        let mut lxt = PeekableLexer::from(test_str);
        assert_eq!(Some(Token::ID(test_str)), lxt.next());
    }

    #[test]
    fn lexer_peek_index_1_test() {
        let solution = vec!["big", "kahuna", "electric", "boogaloo"];
        let test_text: String = solution
            .iter()
            .map(|x| String::from(*x) + " ")
            .collect::<Vec<String>>()
            .iter()
            .map(|x| x.chars())
            .flatten()
            .collect();

        let mut lexer_to_test = PeekableLexer::from(&test_text);
        for _val in solution {
            let v1 = lexer_to_test.peek().unwrap().clone();
            let v2 = lexer_to_test.next().clone().unwrap();
            assert_eq!(v1, v2);
        }
    }

    #[test]
    fn lexer_no_semicolon_test() {
        let test_string = "
            hi
            there
        ";
        let mut lexer = PeekableLexer::from(test_string);
        println!("{}", test_string);
        assert_eq!(lexer.next(), Some(Token::NewLine));
        assert_eq!(lexer.next(), Some(Token::ID("hi")));
        assert_eq!(lexer.next(), Some(Token::NewLine));

        assert_eq!(lexer.next(), Some(Token::ID("there")));
        assert_eq!(lexer.next(), Some(Token::NewLine));
    }

    #[test]
    fn lexer_comments() {
        let test_string = "
            hi // hello
            /* a */ there /***/
            /*//*/a/*/* /*/b
            c/*
            d
          */e";
        let mut lexer = PeekableLexer::from(test_string);
        println!("{}", test_string);
        assert_eq!(lexer.next(), Some(Token::NewLine));
        assert_eq!(lexer.next(), Some(Token::ID("hi")));
        assert_eq!(lexer.next(), Some(Token::NewLine));

        assert_eq!(lexer.next(), Some(Token::ID("there")));
        assert_eq!(lexer.next(), Some(Token::NewLine));

        assert_eq!(lexer.next(), Some(Token::ID("a")));
        assert_eq!(lexer.next(), Some(Token::ID("b")));
        assert_eq!(lexer.next(), Some(Token::NewLine));
        assert_eq!(lexer.next(), Some(Token::ID("c")));
        assert_eq!(lexer.next(), Some(Token::ID("e")));
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn lexer_slice_indexing_1_test() {
        let solution = vec!["big ", "kahuna ", "electric ", "boogaloo "];
        let test_text: String = solution.iter().map(|x| x.chars()).flatten().collect();
        let mut lexer_to_test = PeekableLexer::from(&test_text);

        for sol in solution {
            let _v = lexer_to_test.next();
            let _j = lexer_to_test.peek();
            assert_eq!(lexer_to_test.slice(), sol.trim());
        }
    }

    #[test]
    fn lexer_slice_indexing_2_test() {
        let solution = vec!["big ", "kahuna ", "electric ", "boogaloo "];
        let test_text: String = solution.iter().map(|x| x.chars()).flatten().collect();
        let mut lexer_to_test = PeekableLexer::from(&test_text);

        for sol in solution {
            let _v = lexer_to_test.next();
            assert_eq!(lexer_to_test.slice(), sol.trim());
        }
    }
}
