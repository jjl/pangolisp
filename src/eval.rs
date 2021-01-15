// use crate::forms::*;
// use crate::spans::*;
use crate::exprs::*;
use im::{HashMap, Vector};
// use std::mem::take;
// use std::hash::Hash;

pub enum EvalError {
    StackUnderflow(Stack),
    UnknownBinding(String),
}

#[derive(Clone, Eq, PartialEq)]
pub struct Eval {
    stack: Stack,
}

impl Eval {
    pub fn expand_once(&mut self, expr: Expr) -> Result<Expr, EvalError> {
        Ok(expr)
    }

    pub fn expand(&mut self, expr: Expr) -> Result<Expr, EvalError> {
        loop {
            let exp = self.expand_once(expr.clone())?;
            if expr != exp {
                return Ok(exp);
            }
        }
    }

    // pub fn eval(&mut self, mut expr: Expr) -> Result<Expr, EvalError> {
    //     match expr {
    //         Expr::Nil => Ok(expr),
    //         Expr::Int(_) => Ok(expr),
    //         // Expr::Float(_) => Ok(expr),
    //         Expr::Atom(atom) => self.stack.lookup(&atom).map(|e| e.clone()),
    //         // Expr::String(string) => Ok(expr),
    //         Expr::List(mut list) => {
    //             if list.is_empty() {
    //                 Ok(expr)
    //             } else {
    //                 for l in list.iter_mut() {
    //                     *l = self.eval(take(l))?;
    //                 }
    //                 let head = list.pop_front();
    //             }
    //         }
    //         Expr::Map(_) => Ok(expr),
    //         Expr::Fun(_) => Ok(expr),
    //     }
    // }
}

#[derive(Clone, Default, Eq, PartialEq)]
pub struct Stack {
    current: HashMap<String, Expr>,
    previous: Vector<HashMap<String, Expr>>,
}

impl Stack {
    pub fn push(&mut self) {
        self.previous.push_back(self.current.clone());
    }
    pub fn pop(mut self) -> Result<Self, EvalError> {
        if let Some(locals) = self.previous.pop_back() {
            self.current = locals;
            Ok(self)
        } else {
            Err(EvalError::StackUnderflow(self))
        }
    }
    pub fn assign(&mut self, name: impl Into<String>, value: Expr) {
        self.current.insert(name.into(), value);
    }
    pub fn lookup(&mut self, name: &str) -> Result<&Expr, EvalError> {
        self.current.get(name).ok_or_else(|| EvalError::UnknownBinding(name.to_string()))
    }
}
