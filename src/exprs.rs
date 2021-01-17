use crate::spans::*;
use crate::eval::*;
use std::mem::swap;
use im::{HashMap, Vector};

#[derive(Clone, Default, Eq, Hash, PartialEq)]
pub struct Meta {
    pub span: Option<Span>,
    pub old:  Option<Box<Meta>>,
}

impl Meta {
    pub fn new(span: Span) -> Meta {
        Meta { span: Some(span), old: None }
    }
}

impl From<Span> for Meta {
    fn from(span: Span) -> Meta {
        Meta::new(span)
    }       
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub enum Special {
    // CallWithCurrentContinuation(Meta),
    Lambda(Meta),
    // Match(Meta),
    Quasiquote(Meta),
    Quote(Meta),
    The(Meta),
    Unquote(Meta),
    // UnquoteSplicing(Meta),
}

impl Special {
    pub fn meta(&self) -> &Meta {
        match self {
            Special::Lambda(m) => &m,
            Special::Quasiquote(m) => &m,
            Special::Quote(m) => &m,
            Special::The(m) => &m,
            Special::Unquote(m) => &m,
        }
    }

    // Rust doesn't permit us to return mutable refs to values in an
    // enum and we can't be arsed to use proxy objects, so we do the
    // next best thing and have a setter.
    pub fn set_meta(&mut self, mut meta: Meta) -> Meta {
        match self {
            Special::Lambda(ref mut m) => swap(m, &mut meta),
            Special::Quasiquote(ref mut m) => swap(m, &mut meta),
            Special::Quote(ref mut m) => swap(m, &mut meta),
            Special::The(ref mut m) => swap(m, &mut meta),
            Special::Unquote(ref mut m) => swap(m, &mut meta),
        };
        meta
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct Fun {
    pub param: Box<Symbol>,
    pub body:  Box<Expr>,
    pub meta:  Meta,
}

impl Fun {
    
    pub fn new(param: Box<Symbol>, body:  Box<Expr>, meta:  Meta)  -> Fun {
        Fun { param, body, meta }
    }


    pub fn call(&self, arg: Expr, eval: &mut Eval) -> Result<Expr, EvalError> {
        let mut fval = eval.clone();
        fval.stack.push();
        fval.stack.assign(&self.param.value, arg);
        let result = fval.eval(*self.body.clone())?;
        Ok(result)
    }

}

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct Int {
    pub value: i64,
    pub meta: Meta,
}

impl Int {
    pub fn new(value: i64, meta: Meta) -> Int {
        Int { value, meta }
    }
}

impl From<i64> for Int {
    fn from(value: i64) -> Int {
        Int::new(value, Meta::default())
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct Symbol {
    pub value: String,
    pub meta: Meta,
}

impl Symbol {
    pub fn new(value: String, meta: Meta) -> Symbol {
        Symbol { value, meta }
    }
}

impl From<String> for Symbol {
    fn from(value: String) -> Symbol {
        Symbol::new(value, Meta::default())
    }
}

#[derive(Clone, Default, Eq, Hash, PartialEq)]
pub struct List {
    pub vals: Vector<Expr>,
    pub meta: Meta,
}

impl List {
    pub fn new(vals: Vector<Expr>, meta: Meta) -> List {
        List { vals, meta }
    }
}

impl From<Vector<Expr>> for List {
    fn from(value: Vector<Expr>) -> List {
        List::new(value, Meta::default())
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct Map {
    pub vals: HashMap<Expr, Expr>,
    pub meta: Meta,
}

impl Map {
    pub fn new(vals: HashMap<Expr, Expr>, meta: Meta) -> Map {
        Map { vals, meta }
    }
}

impl From<HashMap<Expr, Expr>> for Map {
    fn from(value: HashMap<Expr, Expr>) -> Map {
        Map::new(value, Meta::default())
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub enum Expr {
    Nil,
    Int(Int),
    Symbol(Symbol),
    // Float(Float),
    // String(Str),
    List(List),
    Map(Map),
    Fun(Fun),
    Macro(Fun),
    Special(Special),
    // Continuation(Stack),
}

impl Expr {
    pub fn meta(&self) -> Option<&Meta> {
        match self {
            Expr::Nil => None,
            Expr::Int(e) => Some(&e.meta),
            Expr::Symbol(e) => Some(&e.meta),
            Expr::List(e) => Some(&e.meta),
            Expr::Map(e) => Some(&e.meta),
            Expr::Fun(e) => Some(&e.meta),
            Expr::Macro(e) => Some(&e.meta),
            Expr::Special(e) => Some(e.meta()),
        }
    }
    pub fn set_meta(&mut self, mut meta: Meta) -> Option<Meta> {
        match self {
            Expr::Nil => return None,
            Expr::Int(ref mut e) => swap(&mut e.meta, &mut meta),
            Expr::Symbol(ref mut e) => swap(&mut e.meta, &mut meta),
            Expr::List(ref mut e) => swap(&mut e.meta, &mut meta),
            Expr::Map(ref mut e) => swap(&mut e.meta, &mut meta),
            Expr::Fun(ref mut e) => swap(&mut e.meta, &mut meta),
            Expr::Macro(ref mut e) => swap(&mut e.meta, &mut meta),
            Expr::Special(ref mut e) => return Some(e.set_meta(meta)),
        };
        Some(meta)
    }
}

impl Default for Expr {
    fn default() -> Expr {
        Expr::Nil
    }
}
