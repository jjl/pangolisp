use crate::*;
use crate::parser::*;
// use im::{HashMap, Vector};
// use std::mem;
// use std::iter::Iterator;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Special {
    // Delay,
    // Force,
    Lambda,
    Quote,
    // Reset,
    // Shift,
    // The,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Scalar {
    Bool(bool),
    // Float(Float),
    Int(i64),
    // Ratio(i64, i64),
    Symbol(Compact),
    Special(Special),
}

pub enum Expr {
    Parsed(Parsed),
    Scalar(Scalar),
    Lexical(Compact),
}

pub enum ReadError {
    
}

pub fn read(p: Parsed) -> Result<Expr, ReadError> {
    match p {
        Parsed::Group(group) => {
            todo!()
        }
        Parsed::Literal(lit) => {
            match lit.inner { // WTF is going on here?
                crate::parser::Literal::Int(i) => Ok(Expr::Scalar(Scalar::Int(i))),
                Literal::Symbol(s) => Ok(Expr::Lexical(s)),
            }
        }
        Parsed::Macro(mac) => {
            todo!()
        }
    }
}
// pub type Env = Stack<Binding>;

// #[derive(Clone, Eq, Hash, PartialEq)]
// pub struct Binding {
//     pub name: String,
//     pub value: Expr,
// }

// impl Binding {
//     pub fn new(name: impl Into<String>, value: Expr) -> Binding {
//         Binding { name: name.into(), value }
//     }
//     pub fn rename(&mut self, to: impl Into<String>) {
//         self.name = to.into();
//     }
//     pub fn replace(&mut self, to: Expr) -> Expr {
//         mem::replace(&mut self.value, to)
//     }
// }

// impl From<Binding> for Expr {
//     fn from(binding: Binding) -> Expr {
//         binding.value
//     }
// }

// #[derive(Clone, Default, Eq, Hash, PartialEq)]
// pub struct Meta {
//     pub span: Option<Span>,
//     pub old:  Option<Box<Meta>>,
// }

// impl Meta {
//     pub fn new(span: Span) -> Meta {
//         Meta { span: Some(span), old: None }
//     }
//     pub fn push_old(&mut self, meta: Meta) {
//         if let Some(old) = &mut self.old {
//             old.push_old(meta);
//         } else {
//             self.old = Some(Box::new(meta));
//         }
//     }
// }

// impl From<Span> for Meta {
//     fn from(span: Span) -> Meta {
//         Meta::new(span)
//     }       
// }

// #[derive(Clone, Eq, Hash, PartialEq)]
// pub struct Fun {
//     pub param:   Param,
//     pub body:    Box<Expr>,
//     pub closure: Env,
//     pub meta:    Meta,
// }

// impl Fun {
//     pub fn new(param: Param, body: Box<Expr>, closure: Env, meta:  Meta)  -> Fun {
//         Fun { param, body, closure, meta }
//     }
// }


// #[derive(Clone, Eq, Hash, PartialEq)]
// pub struct Int {
//     pub value: i64,
//     pub meta: Meta,
// }

// impl Int {
//     pub fn new(value: i64, meta: Meta) -> Int {
//         Int { value, meta }
//     }
// }

// impl From<i64> for Int {
//     fn from(value: i64) -> Int {
//         Int::new(value, Meta::default())
//     }
// }

// #[derive(Clone, Eq, Hash, PartialEq)]
// pub struct Symbol {
//     pub value: String,
//     pub meta: Meta,
// }

// impl Symbol {
//     pub fn new(value: String, meta: Meta) -> Symbol {
//         Symbol { value, meta }
//     }
// }

// impl From<String> for Symbol {
//     fn from(value: String) -> Symbol {
//         Symbol::new(value, Meta::default())
//     }
// }

// #[derive(Clone, Default, Eq, Hash, PartialEq)]
// pub struct List {
//     pub vals: Vector<Expr>,
//     pub meta: Meta,
// }

// impl List {
//     pub fn new(vals: Vector<Expr>, meta: Meta) -> List {
//         List { vals, meta }
//     }
// }

// impl From<Vector<Expr>> for List {
//     fn from(value: Vector<Expr>) -> List {
//         List::new(value, Meta::default())
//     }
// }

// #[derive(Clone, Eq, Hash, PartialEq)]
// pub struct Map {
//     pub vals: HashMap<Expr, Expr>,
//     pub meta: Meta,
// }

// impl Map {
//     pub fn new(vals: HashMap<Expr, Expr>, meta: Meta) -> Map {
//         Map { vals, meta }
//     }
// }

// impl From<HashMap<Expr, Expr>> for Map {
//     fn from(value: HashMap<Expr, Expr>) -> Map {
//         Map::new(value, Meta::default())
//     }
// }

// #[derive(Clone, Eq, Hash, PartialEq)]
// pub enum Expr {
//     Nil,
//     Int(Int),
//     Symbol(Symbol),
//     // Float(Float),
//     // String(Str),
//     List(List),
//     Map(Map),
//     Fun(Fun),
//     Macro(Fun),
//     Fexpr(Fun),
//     Special(Special),
// }

// impl Expr {
//     pub fn meta(&self) -> Option<&Meta> {
//         match self {
//             Expr::Nil => None,
//             Expr::Int(e) => Some(&e.meta),
//             Expr::Symbol(e) => Some(&e.meta),
//             Expr::List(e) => Some(&e.meta),
//             Expr::Map(e) => Some(&e.meta),
//             Expr::Fun(e) => Some(&e.meta),
//             Expr::Macro(e) => Some(&e.meta),
//             Expr::Fexpr(e) => Some(&e.meta),
//             Expr::Special(e) => Some(e.meta()),
//         }
//     }
//     pub fn meta_mut(&mut self) -> Option<&mut Meta> {
//         match self {
//             Expr::Nil => None,
//             Expr::Int(e) => Some(&mut e.meta),
//             Expr::Symbol(e) => Some(&mut e.meta),
//             Expr::List(e) => Some(&mut e.meta),
//             Expr::Map(e) => Some(&mut e.meta),
//             Expr::Fun(e) => Some(&mut e.meta),
//             Expr::Macro(e) => Some(&mut e.meta),
//             Expr::Fexpr(e) => Some(&mut e.meta),
//             Expr::Special(e) => Some(e.meta_mut()),
//         }
//     }
// }

// impl Default for Expr {
//     fn default() -> Expr {
//         Expr::Nil
//     }
// }

// impl From<Special> for Expr {
//     fn from(special: Special) -> Expr {
//         Expr::Special(special)
//     }
// }
