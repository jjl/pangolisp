// #[macro_use]
// extern crate derive_builder;

pub mod spans;
pub mod tokens;
pub mod forms;
pub mod exprs;
pub mod reader;
pub mod eval;

// #[derive(Clone, Eq, PartialEq)]
// pub enum Kind {
//     Star,
//     Arrow(Box<Kind>, Box<Kind>),
// }

// pub mod types {
//     use super::*;
//     pub fn unit() -> Type {
//         TypeConstructor::new("()", Kind::Star).into()
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
