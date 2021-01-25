use crate::*;
use crate::tokens::*;
use crate::lists::*;
use im::Vector;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Group {
    pub open:  Spanning<Paren>,
    pub close: Spanning<Paren>,
    pub vals:  Vector<Parsed>,
}

impl Group {
    pub fn span(&self) -> Span {
        self.open.span.start.span(self.close.span.end)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Literal {
    Int(i64),
    // Float(OrderedFloat<f64>),
    Symbol(Compact),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Parsed {
    Group(Group),
    Literal(Spanning<Literal>),
    Macro(Box<ParserMacro>),
}

impl Parsed {
    pub fn span(&self) -> Span {
        match self {
            Parsed::Group(g) => g.span(),
            Parsed::Literal(l) => l.span,
            Parsed::Macro(m) => (*m).span(),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ParserMacro {
    Lambda(Span, Spanning<Parsed>, Spanning<Parsed>),
    Quasi(Span, Spanning<Parsed>),
    Quote(Span, Spanning<Parsed>),
    The(Span, Spanning<Parsed>, Spanning<Parsed>),
    Unquote(Span, Spanning<Parsed>),
}

impl ParserMacro {
    pub fn span(&self) -> Span {
        match self {
            ParserMacro::Lambda(open, _, body) => open.start.span(body.span.end),
            ParserMacro::Quasi(open, val) => open.start.span(val.span.end),
            ParserMacro::Quote(open, val) => open.start.span(val.span.end),
            ParserMacro::The(open, _, val) => open.start.span(val.span.end),
            ParserMacro::Unquote(open, val) => open.start.span(val.span.end),
        }
    }
}

/// A Parsed that is still being parsed, a partial parsed.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Parsing {
    Group(Spanning<Paren>, Vector<Parsed>),
    Lambda(Spanning<Prefix>, Option<Parsed>),
    The(Spanning<Prefix>, Option<Parsed>),
    Quoting(Spanning<Prefix>),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ParseError {
    /// An error tokenising.
    Token(TokenError),
    /// Oops, mismatched closing delimiter.
    DoesNotComplete(Spanning<Paren>, Vector<Parsing>),
    /// No more code, but stuff is still open.
    Incomplete(Vector<Parsing>),
}

pub struct Parser<'a> {
    tokens: Tokens<'a>,
    stack: Vec<Parsing>,
}

// todo: port me
// impl<'a> Forms<'a> {
//     pub fn new(source: &'a str) -> Self {
//         Forms { tokens: Tokens::new(source), partials: Vector::new() }
//     }

//     fn does_not_complete(&mut self, span: Span, paren: Paren) ->  Result<Form<'a>, FormError<'a>> {
//         let span = Spanning::new(paren, span);
//         Err(FormError::DoesNotComplete(span, take(&mut self.partials)))
//     }

//     fn push(&mut self, token: Spanning<Token<'a>>) {
//         let partial =
//             match token.inner {
//                 Token::Open(open) =>
//                     Partial::Group(Spanning::new(open, token.span), Vector::new()),
//                 Token::Prefix(prefix) =>
//                     if prefix == Prefix::The {
//                         Partial::The(Spanning::new(prefix, token.span), None)
//                     } else {
//                         Partial::Quoting(Spanning::new(prefix, token.span))
//                     },
//                 _ => unreachable!(),                    
//             };
//         self.partials.push_back(partial);
//     }

//     fn close(&mut self, token: Spanning<Token<'a>>) -> Result<Form<'a>, FormError<'a>> {
//         if let Token::Close(close) = token.inner {
//             match self.partials.back() {
//                 Some(Partial::Group(open, _)) => {
//                     if open.inner == close {
//                         if let Partial::Group(open, vals) = self.partials.pop_back().unwrap() {
//                             let group = Group { open, close: Spanning::new(close, token.span), vals };
//                             Ok(Form::Group(group))
//                         } else { unreachable!() }
//                     } else {
//                         self.does_not_complete(token.span, close)
//                     }
//                 }
//                 Some(_) => self.does_not_complete(token.span, close),
//                 None => Err(FormError::Incomplete(take(&mut self.partials))),
//             }
//         } else { unreachable!() }
//     }

//     fn literal(&mut self, token: Spanning<Token<'a>>) -> Result<Form<'a>, FormError<'a>> {
//         if let Token::Literal(l) = token.inner {
//             match l {
//                 Literal::Int(int)    => Ok(Form::Int(Spanning::new(int, token.span))),
//                 Literal::Symbol(sym) => Ok(Form::Symbol(Spanning::new(sym, token.span))),
//             }
//         } else { unreachable!() }
//     }

//     fn next_form(&mut self) -> Option<Result<Form<'a>, FormError<'a>>> {
//         loop {
//             match self.tokens.next() {
//                 Some(Ok(token)) => {
//                     match token.inner {
//                         Token::Open(_) => { self.push(token); }
//                         Token::Prefix(_) => { self.push(token); }
//                         Token::Close(_) => return Some(self.close(token)),
//                         Token::Literal(_) => return Some(self.literal(token)), 
//                         // Token::String(s) => self.list(Spanning::new(s, token.span), open),
//                         _ => {}
//                     }
//                 }
//                 Some(Err(e)) => return Some(Err(FormError::Token(e))),
//                 None => {
//                     return
//                         if self.partials.is_empty() { None }
//                     else { Some(Err(FormError::Incomplete(take(&mut self.partials)))) };
//                 }
//             }
//         }
//     }

//     fn form(&mut self) -> Option<Result<Form<'a>, FormError<'a>>> {
//         loop {
//             match self.next_form() {
//                 Some(Ok(form)) => {
//                     let mut form = Some(form);
//                     loop {
//                         match (form.take(), self.partials.pop_back()) {
//                             (Some(gorm), Some(mut partial)) => {
//                                 match partial {
//                                     Partial::The(prefix, Some(typ)) => {
//                                         form = Some(Form::Macro(Macro::The(prefix, Box::new(typ), Box::new(gorm))));
//                                     }
//                                     Partial::The(_, ref mut none) => {
//                                         *none = Some(gorm);
//                                         self.partials.push_back(partial);
//                                     }
//                                     Partial::Quoting(prefix) => {
//                                         let macr =
//                                             match prefix.inner {
//                                                 Prefix::Quasiquote => Macro::Quasiquote(prefix, Box::new(gorm)),
//                                                 Prefix::Quote => Macro::Quote(prefix, Box::new(gorm)),
//                                                 _ => Macro::Unquote(prefix, Box::new(gorm)),
//                                             };
//                                         form = Some(Form::Macro(macr));
//                                     }
//                                     Partial::Group(_open, ref mut vals) => {
//                                         vals.push_back(gorm);
//                                         self.partials.push_back(partial);
//                                     }
//                                 }
//                             }
//                             (Some(gorm), None) => return Some(Ok(gorm)),
//                             (None, Some(partial)) => { self.partials.push_back(partial); }
//                             (None, None) => break,
//                         }
//                     }
//                 }
//                 other => return other,
//             }
//         }
//     }
// }

// impl<'a> Iterator for Parser<'a> {
//     type Item = Result<Parsed, ParseError>;
//     fn next(&mut self) -> Option<Self::Item> {
//         self.form()
//     }
// }

