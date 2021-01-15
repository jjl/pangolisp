use crate::spans::*;
use im::{HashMap, Vector};

#[derive(Clone, Default, Eq, Hash, PartialEq)]
pub struct Meta {
    pub span: Option<Span>,
    pub old: Option<Box<Meta>>,
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
    Lambda(Meta),
    Let(Meta),
    Match(Meta),
    Quasiquote(Meta),
    Quote(Meta),
    The(Meta),
    Unquote(Meta),
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub enum Builtin {
    List(Meta),
    Map(Meta),
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct Param {
    pub name: String,
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub enum FunKind {
    Function,
    Macro,
    Fexpr,
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct Fun {
    pub kind:  FunKind,
    pub param: Param,
    pub body:  Box<Expr>,
}

impl Fun {
    
//     // pub fn call(&self, args: Vector<Expr>, eval: &mut Eval) -> Result<Expr, EvalError> {
//     //     self.assign_args(args)?;
//     //     unimplemented!()
//     // }
//     fn assign_args(&self, args: Vector<Expr>) -> Result<(), EvalError> {
//         let mut params = self.params.iter();
//         let mut args = args.into_iter();
//         loop {
//             match (params.next(), args.next()) {
//                 (None, None) => { return Ok(()); }
//                 (None, Some(arg)) => {}
//                 (Some(param), None) => {}
//                 (Some(param), Some(arg)) => {}
//             }
//         }
//     }
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

#[derive(Clone, Eq, Hash, PartialEq)]
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
    Special(Special),
}
