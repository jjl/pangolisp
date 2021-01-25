use crate::spans::*;
// use ordered_float::OrderedFloat;
use std::convert::TryFrom;
use std::hash::Hash;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum TokenError {
    InvalidChar(char),
    Partial,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Paren {
    Paren,
    Brace,
    Square,
}

impl Paren {
    pub fn open(self) -> char {
        match self {
            Paren::Paren => '(',
            Paren::Brace => '}',
            Paren::Square => '[',
        }
    }

    pub fn close(self) -> char {
        match self {
            Paren::Paren => ')',
            Paren::Brace => '}',
            Paren::Square => ']',
        }
    }
}

impl TryFrom<char> for Paren {
    type Error = ();
    fn try_from(ch: char) -> Result<Self, ()> {
        match ch {
            '(' => Ok(Paren::Paren),
            ')' => Ok(Paren::Paren),
            '{' => Ok(Paren::Brace),
            '}' => Ok(Paren::Brace),
            '[' => Ok(Paren::Square),
            ']' => Ok(Paren::Square),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Prefix {
    HasType,
    Lambda,
    Quasiquote,
    Quote,
    Unquote,
}

impl TryFrom<char> for Prefix {
    type Error = ();
    fn try_from(ch: char) -> Result<Self, ()> {
        match ch {
            '\\' => Ok(Prefix::Lambda),
            ':'  => Ok(Prefix::HasType),
            ')'  => Ok(Prefix::Quasiquote),
            '{'  => Ok(Prefix::Quote),
            '}'  => Ok(Prefix::Unquote),
            _    => Err(()),
        }
    }
}

// #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
// pub enum StringEscape {
//     DoubleQuote,
//     Newline,
//     Backslash,
// }

// #[derive(Clone, Debug, Eq, Hash, PartialEq)]
// pub enum StringToken<'a> {
//     Delimiter,
//     Text(&'a str),
//     Escape(StringEscape),
// }

// impl<'a> StringToken<'a> {
//     pub fn may_continue(&self) -> bool {
//         match self {
//             StringToken::Delimiter => false,
//             StringToken::Text(_) => true,
//             StringToken::Escape(_) => false,
//         }
//     }
// }

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Literal<'a> {
    Int(i64),
    // Float(OrderedFloat<f64>),
    Symbol(&'a str),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Token<'a> {
    Literal(Literal<'a>),
    Open(Paren),
    Close(Paren),
    Prefix(Prefix),
    // String(StringToken<'a>)
    Whitespace(&'a str),
}

impl<'a> Token<'a> {
    pub fn may_continue(&self) -> bool {
        match self {
            Token::Literal(_) => true,
            Token::Open(_) => false,
            Token::Close(_) => false,
            Token::Prefix(_) => false,
            Token::Whitespace(_) => true,
            // Token::String(t) => t.may_continue(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Tokens<'a> {
    in_string: bool,
    source: &'a str,
    pub pos: Pos,
}

impl<'a> Tokens<'a> {
    pub fn new(source: &'a str) -> Self {
        Tokens { source, pos: Pos::default(), in_string: false }
    }

    pub fn at_end(&self) -> bool {
        self.source.is_empty()
    }

    fn span_move_cols(&mut self, cols: usize) -> Span {
        let start = self.pos;
        self.pos = start.advance_columns(cols);
        Span::new(start, self.pos)
    }

    fn spanning_move_cols(&mut self, chars: usize, token: Token<'a>) -> Spanning<Token<'a>> {
        let span = self.span_move_cols(chars);
        Spanning::new(token, span)
    }

    fn parse_number(&mut self) -> Spanning<Token<'a>> {
        if self.source.starts_with("0x") {
            let (_, after) = self.source.split_at(2);
            match after.find(|ch: char| !ch.is_ascii_hexdigit()) {
                Some(index) => {
                    let (before, after) = after.split_at(index);
                    self.source = after;
                    let start = self.pos;
                    let end = start.advance_columns(before.len() + 2);
                    let span = Span::new(start, end);
                    let token = Token::Literal(Literal::Int(i64::from_str_radix(before, 16).unwrap()));
                    self.pos = end;
                    Spanning::new(token, span)
                }
                None => {
                    let start = self.pos;
                    let end = start.advance_columns(self.source.len());
                    let span = Span::new(start, end);
                    let token = Token::Literal(Literal::Int(i64::from_str_radix(self.source, 16).unwrap()));
                    self.source = "";
                    self.pos = end;
                    Spanning::new(token, span)
                }
            }
        } else {
            match self.source.find(|ch: char| !ch.is_ascii_digit()) {
                Some(index) => {
                    let (before, after) = self.source.split_at(index);
                    self.source = after;
                    let start = self.pos;
                    let end = start.advance_columns(before.len());
                    let span = Span::new(start, end);
                    let token = Token::Literal(Literal::Int(i64::from_str_radix(before, 10).unwrap()));
                    self.pos = end;
                    Spanning::new(token, span)
                }
                None => {
                    let start = self.pos;
                    let end = start.advance_columns(self.source.len());
                    let span = Span::new(start, end);
                    let token = Token::Literal(Literal::Int(i64::from_str_radix(self.source, 10).unwrap()));
                    self.source = "";
                    self.pos = end;
                    Spanning::new(token, span)
                }
            }
        }
    }

    fn parse_whitespace(&mut self) -> Spanning<Token<'a>> {
        match self.source.find(|ch: char| !ch.is_ascii_whitespace()) {
            Some(index) => {
                let (before, after) = self.source.split_at(index);
                self.source = after;
                let start = self.pos;
                let end = start.after(before);
                let span = Span::new(start, end);
                let token = Token::Whitespace(before.into());
                self.pos = end;
                Spanning::new(token, span)
            }
            None => {
                let start = self.pos;
                let end = start.after(self.source);
                let span = Span::new(start, end);
                let token = Token::Whitespace(self.source.into());
                self.source = "";
                self.pos = end;
                Spanning::new(token, span)
            }
        }
    }
    // TODO: dots between indicates an accessor
    fn parse_symbol(&mut self) -> Spanning<Token<'a>> {
        match self.source.find(is_symbol) {
            Some(index) => {
                let (before, after) = self.source.split_at(index);
                self.source = after;
                let start = self.pos;
                let end = start.after(before);
                let span = Span::new(start, end);
                let token = Token::Literal(Literal::Symbol(before.into()));
                self.pos = end;
                Spanning::new(token, span)
            }
            None => {
                let start = self.pos;
                let end = start.after(self.source);
                let span = Span::new(start, end);
                let token = Token::Literal(Literal::Symbol(self.source.into()));
                self.source = "";
                self.pos = end;
                Spanning::new(token, span)
            }
        }
    }

    fn parse_string_token(&mut self) -> Option<Result<Spanning<Token<'a>>, TokenError>> {
        unimplemented!()
    }

    fn parse_program_token(&mut self) -> Option<Result<Spanning<Token<'a>>, TokenError>> {
        let ch = self.source.chars().next()?;
        if ch.is_ascii_digit() {
            Some(Ok(self.parse_number()))
        } else if ch.is_control() {
            Some(Err(TokenError::InvalidChar(ch)))
        } else if ch.is_ascii_whitespace() {
            Some(Ok(self.parse_whitespace()))
        } else if is_prefix(ch) {
            Some(Ok(self.spanning_move_cols(1, Token::Prefix(Prefix::try_from(ch).unwrap()))))
        } else if is_open(ch) {
            Some(Ok(self.spanning_move_cols(1, Token::Open(Paren::try_from(ch).unwrap()))))
        } else if is_close(ch) {
            Some(Ok(self.spanning_move_cols(1, Token::Close(Paren::try_from(ch).unwrap()))))
        } else {
            Some(Ok(self.parse_symbol()))
        }
    }
}

impl<'a> Iterator for Tokens<'a> {
    type Item = Result<Spanning<Token<'a>>, TokenError>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.in_string {
            self.parse_string_token()
        } else {
            self.parse_program_token()
        }
    }
}

fn is_open(ch: char) -> bool {
    "({[".chars().any(|dh| ch == dh)
}

fn is_close(ch: char) -> bool {
    ")}]".chars().any(|dh| ch == dh)
}

fn is_prefix(ch: char) -> bool {
    "\\:`'~".chars().any(|dh| ch == dh)
}

fn is_symbol(ch: char) -> bool {
    !is_prefix(ch) && !is_open(ch) && !is_close(ch)
        && !ch.is_ascii_digit() && !ch.is_ascii_whitespace() && !ch.is_control()
}
