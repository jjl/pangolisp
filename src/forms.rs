use std::mem::take;
use std::borrow::Cow;
use im::Vector;
use crate::spans::*;
use crate::tokens::*;
use std::ops::Deref;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Group<T: Clone> {
    pub open:  Spanning<Paren>,
    pub close: Spanning<Paren>,
    pub vals:  Vector<T>,
}

impl<T: Clone> Group<T> {
    pub fn span(&self) -> Span {
        self.open.span.start.span(self.close.span.end)
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Macro<'a>  {
    HasType(Spanning<Prefix>, Box<Form<'a>>, Box<Form<'a>>),
    Quasiquote(Spanning<Prefix>, Box<Form<'a>>),
    Quote(Spanning<Prefix>, Box<Form<'a>>),
    Unquote(Spanning<Prefix>, Box<Form<'a>>),
}

impl<'a> Macro<'a> {
    pub fn span(&self) -> Span {
        match self {
            Macro::HasType(prefix, _typ, form) => prefix.span.start.span(form.deref().span().end),
            Macro::Quasiquote(prefix, form) => prefix.span.start.span(form.span().end),
            Macro::Quote(prefix, form) => prefix.span.start.span(form.span().end),
            Macro::Unquote(prefix, form) => prefix.span.start.span(form.span().end),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum FormError<'a> {
    Token(TokenError),
    DoesNotComplete(Spanning<Paren>, Vector<Partial<Form<'a>>>),
    Incomplete(Vector<Partial<Form<'a>>>),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Form<'a> {
    Macro(Macro<'a>),
    Group(Group<Form<'a>>),
    Int(Spanning<i64>),
    Symbol(Spanning<Cow<'a, str>>),
}

impl<'a> Form<'a> {
    pub fn span(&self) -> Span {
        match self {
            Form::Macro(macr) => macr.span(),
            Form::Group(group) => group.span(),
            Form::Int(int) => int.span,
            Form::Symbol(sym) => sym.span,
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Partial<T: Clone> {
    HasType(Spanning<Prefix>, Option<T>),
    Quoting(Spanning<Prefix>),
    Group(Spanning<Paren>, Vector<T>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Forms<'a> {
    tokens: Tokens<'a>,
    partials: Vector<Partial<Form<'a>>>,
}

impl<'a> Forms<'a> {
    pub fn new(source: &'a str) -> Self {
        Forms { tokens: Tokens::new(source), partials: Vector::new() }
    }

    fn does_not_complete(&mut self, span: Span, paren: Paren) ->  Result<Form<'a>, FormError<'a>> {
        let span = Spanning::new(paren, span);
        Err(FormError::DoesNotComplete(span, take(&mut self.partials)))
    }

    fn push(&mut self, token: Spanning<Token<'a>>) {
        let partial =
            match token.inner {
                Token::Open(open) =>
                    Partial::Group(Spanning::new(open, token.span), Vector::new()),
                Token::Prefix(prefix) =>
                    if prefix == Prefix::HasType {
                        Partial::HasType(Spanning::new(prefix, token.span), None)
                    } else {
                        Partial::Quoting(Spanning::new(prefix, token.span))
                    },
                _ => unreachable!(),                    
            };
        self.partials.push_back(partial);
    }

    fn close(&mut self, token: Spanning<Token<'a>>) -> Result<Form<'a>, FormError<'a>> {
        if let Token::Close(close) = token.inner {
            match self.partials.back() {
                Some(Partial::Group(open, _)) => {
                    if open.inner == close {
                        if let Partial::Group(open, vals) = self.partials.pop_back().unwrap() {
                            let group = Group { open, close: Spanning::new(close, token.span), vals };
                            Ok(Form::Group(group))
                        } else { unreachable!() }
                    } else {
                        self.does_not_complete(token.span, close)
                    }
                }
                Some(_) => self.does_not_complete(token.span, close),
                None => Err(FormError::Incomplete(take(&mut self.partials))),
            }
        } else { unreachable!() }
    }

    fn literal(&mut self, token: Spanning<Token<'a>>) -> Result<Form<'a>, FormError<'a>> {
        if let Token::Literal(l) = token.inner {
            match l {
                Literal::Int(int)    => Ok(Form::Int(Spanning::new(int, token.span))),
                Literal::Symbol(sym) => Ok(Form::Symbol(Spanning::new(sym, token.span))),
            }
        } else { unreachable!() }
    }

    fn next_form(&mut self) -> Option<Result<Form<'a>, FormError<'a>>> {
        loop {
            match self.tokens.next() {
                Some(Ok(token)) => {
                    match token.inner {
                        Token::Open(_) => { self.push(token); }
                        Token::Prefix(_) => { self.push(token); }
                        Token::Close(_) => return Some(self.close(token)),
                        Token::Literal(_) => return Some(self.literal(token)), 
                        // Token::String(s) => self.list(Spanning::new(s, token.span), open),
                        _ => {}
                    }
                }
                Some(Err(e)) => return Some(Err(FormError::Token(e))),
                None => {
                    return
                        if self.partials.is_empty() { None }
                    else { Some(Err(FormError::Incomplete(take(&mut self.partials)))) };
                }
            }
        }
    }

    fn form(&mut self) -> Option<Result<Form<'a>, FormError<'a>>> {
        loop {
            match self.next_form() {
                Some(Ok(form)) => {
                    let mut form = Some(form);
                    loop {
                        match (form.take(), self.partials.pop_back()) {
                            (Some(gorm), Some(mut partial)) => {
                                match partial {
                                    Partial::HasType(prefix, Some(typ)) => {
                                        form = Some(Form::Macro(Macro::HasType(prefix, Box::new(typ), Box::new(gorm))));
                                    }
                                    Partial::HasType(_, ref mut none) => {
                                        *none = Some(gorm);
                                        self.partials.push_back(partial);
                                    }
                                    Partial::Quoting(prefix) => {
                                        let macr =
                                            match prefix.inner {
                                                Prefix::Quasiquote => Macro::Quasiquote(prefix, Box::new(gorm)),
                                                Prefix::Quote => Macro::Quote(prefix, Box::new(gorm)),
                                                _ => Macro::Unquote(prefix, Box::new(gorm)),
                                            };
                                        form = Some(Form::Macro(macr));
                                    }
                                    Partial::Group(_open, ref mut vals) => {
                                        vals.push_back(gorm);
                                        self.partials.push_back(partial);
                                    }
                                }
                            }
                            (Some(gorm), None) => return Some(Ok(gorm)),
                            (None, Some(partial)) => { self.partials.push_back(partial); }
                            (None, None) => break,
                        }
                    }
                }
                other => return other,
            }
        }
    }
}

impl<'a> Iterator for Forms<'a> {
    type Item = Result<Form<'a>, FormError<'a>>;
    fn next(&mut self) -> Option<Self::Item> {
        self.form()
    }
}
