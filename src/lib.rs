#[macro_use]
extern crate slog;

pub mod compactor; // string interning
pub mod spans; // positions for tokens
pub mod tokens; // tokenisation
pub mod parser; // parsing
pub mod exprs; // the everything after parsing type
pub mod lists;
// pub mod forms;
// pub mod exprs;
// pub mod eval;
pub mod stack;

// pub mod vm;

use compactor::*;
use spans::*;

#[derive(Debug, Default, Hash)]
struct Fresh(usize);

impl Fresh {
    #[inline(always)]
    pub fn next(&mut self) -> usize {
        self.0 += 1;
        self.0
    }
}

#[derive(Clone, Default, Eq, Hash, PartialEq)]
pub struct Meta {
    pub defined_at: Option<Span>,
}

// #[derive(Clone, Eq, PartialEq)]
// pub enum Kind {
//     Star,
//     Arrow(Box<Kind>, Box<Kind>),
// }

// pub mod types {
//     use super::*;
//     pub fn unit() -> Type {
//         TypeConstructor::new("()", Kind::Star).into
//     }
// }

// #[derive(Clone, Eq, PartialEq)]
// pub struct TypeConstructor {
//     pub name: String,
//     pub kind: Kind,
// }

// impl TypeConstructor {
//     pub fn new(name: impl Into<String>, kind: Kind) -> Self {
//         TypeConstructor { name: name.into(), kind }
//     }
// }

// #[derive(Clone, Eq, PartialEq)]
// pub struct TypeVar {
//     pub name: String,
//     pub kind: Kind,
// }

// impl TypeVar {
//     pub fn new(name: impl Into<String>, kind: Kind) -> Self {
//         TypeVar { name: name.into(), kind }
//     }
// }

// #[derive(Clone, Eq, PartialEq)]
// pub enum Type {
//     Var(TypeVar),
//     Constructor(TypeConstructor),
//     Application(Box<Type>, Box<Type>),
//     // Generic(usize),
// }

// impl From<TypeVar> for Type {
//     fn from(var: TypeVar) -> Type {
//         Type::Var(var)
//     }
// }

// impl From<TypeConstructor> for Type {
//     fn from(ctor: TypeConstructor) -> Type {
//         Type::Constructor(ctor)
//     }
// }

// impl Type {
//     pub fn apply(self, to: impl Into<Box<Type>>) -> Self {
//         Type::Application(Box::new(self), to.into())
//     }
//     pub fn kind(&self) -> &Kind {
//         match self {
//             Type::Var(v) => &v.kind,
//             Type::Constructor(c) => &c.kind,
//             Type::Application(l, _) => {
//                 match l.kind() {
//                     Kind::Arrow(_, kind) => &*kind,
//                     _ => unreachable!(),
//                 }
//             }
//         }
//     }
// }
