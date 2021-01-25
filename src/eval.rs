// use crate::forms::*;
// use crate::spans::*;
use crate::exprs::*;
use crate::stack::*;
use std::cell::RefCell;
use std::rc::Rc;
use im::Vector;

pub enum Sexp {
}

pub enum EvalError {
    BadParameter(&'static str, Expr),
    ExtraArguments(Expr, List),
    MissingArguments(Expr, usize),
    NotCallable(Expr, Expr, Env),
    UnknownBinding(String, Env),
    UnexpandedMacro(Fun),
}

// pub struct Errors {
    
// }

pub type Evaled = Result<Expr, EvalError>;

#[derive(Clone, Eq, PartialEq)]
pub struct Cell {
    inner: Rc<RefCell<Expr>>,
}

impl Cell {
    pub fn new(expr: Expr) -> Cell {
        Cell { inner: Rc::new(RefCell::new(expr)) }
    }
    pub fn replace(&mut self, other: Expr) -> Expr {
        self.inner.replace(other)
    }
    pub fn replace_with(&mut self, with: impl FnOnce(&mut Expr) -> Expr) -> Expr {
        self.inner.replace_with(with)
    }
    pub fn swap(&mut self, other: &mut Cell) {
        self.inner.swap(&mut other.inner)
    }
    pub fn clone_out(&self) -> Expr {
        (*self.inner.borrow()).clone()
    }
}

fn call_fun(f: Fun, arg: Expr, env: &mut Env) -> Evaled {
    env.enter();
    env.push(Binding::new(f.param.name, arg));
    let ret = eval((*f.body).clone(), env)?;
    env.leave();
    Ok(ret)
}

fn call_one(fun: Expr, arg: Expr, env: &mut Env) -> Evaled {
    match fun {
        Expr::Fun(f) => call_fun(f, eval(arg, env)?, env),
        Expr::Fexpr(f) => call_fun(f, arg, env),
        _ => Err(EvalError::NotCallable(fun, arg, env.clone())),
    }
}

// fn assert_arity(special: &Special, 

fn missing_args(expr: impl Into<Expr>, count: usize) -> EvalError {
    EvalError::MissingArguments(expr.into(), count)
}

fn extra_args(expr: impl Into<Expr>, mut list: Vector<Expr>, meta: Meta, drop: usize) -> EvalError {
    list.truncate(list.len() - drop);
    EvalError::ExtraArguments(expr.into(), List::new(list, meta))
}

fn call_special(special: Special, mut list: Vector<Expr>, meta: Meta, env: &mut Env) -> Evaled {
    match special {
        Special::Lambda(_) => todo!(),
        // Special::Match(meta) => todo!(),
        Special::Quasiquote(_) => todo!(),
        Special::Quote(_) => {
            match list.len() {
                0 => Err(missing_args(special, 1)),
                1 => Ok(list.pop_front().unwrap()),
                n => Err(extra_args(special, list, meta, n)),
            }
        }
        Special::Reset(_) => todo!(),
        Special::Shift(_) => todo!(),
        Special::The(_) => todo!(),
        Special::Unquote(_) => todo!(),
    }
}

fn call_expr(expr: Expr, list: Vector<Expr>, meta: Meta, env: &mut Env) -> Evaled {
    if let Expr::Special(s) = expr {
        call_special(s, list, meta, env)
    } else {
        list.into_iter().try_fold(expr, |fun, arg| {
            let arg = eval(arg, env)?;
            call_one(fun, arg, env)
        })
    }
}

pub fn eval(expr: Expr, env: &mut Env) -> Evaled {
   match expr {
       Expr::Symbol(sym) => {
           match env.lookup(|binding| binding.name == sym.value) {
               Some((_, binding)) => {
                   let mut v = binding.value.clone();
                   v.meta_mut().map(|meta| meta.push_old(sym.meta));
                   Ok(v)
               }
               None => Err(EvalError::UnknownBinding(sym.value.clone(), env.clone())),
           }
       }
       Expr::List(list) => {
           let mut l = list.vals.clone();
           match l.pop_front() {
               Some(front) => {
                   let front = eval(front, env)?;
                   call_expr(front, l, list.meta.clone(), env)
               }
               None => Ok(Expr::List(list)),
           }
       }
       _ => Ok(expr),
   }
}

// impl Eval {

//     fn eval_macro(&mut self, m: Fun, mut list: List) -> Evaled {
//         list.vals.pop_front();
//         m.call(Expr::List(list), self)
//     }

//     pub fn expand_once(&mut self, expr: impl Into<Expr>) -> Evaled {
//         let expr = expr.into();
//         if let Expr::List(list) = expr {
//             match list.vals.front() {
//                 Some(Expr::Macro(m)) => self.eval_macro(m.clone(), list),
//                 Some(Expr::Symbol(s)) => {
//                     if let Some((_, (_, Expr::Macro(m)))) = self.stack.lookup(|(name,_)| name == &s.value) {
//                         let m = m.clone();
//                         self.eval_macro(m, list)
//                     } else {
//                         Ok(Expr::List(list))
//                     }
//                 }
//                 _ => Ok(Expr::List(list)),
//             }
//         } else {
//             Ok(expr)
//         }
//     }

//     pub fn expand(&mut self, expr: Expr) -> Evaled {
//         loop {
//             let exp = self.expand_once(expr.clone())?;
//             if expr == exp {
//                 return Ok(exp);
//             }
//         }
//     }

//     fn eval_lambda(&mut self, meta: Meta, list: List) -> Evaled {
//         let mut l = list.clone();
//         l.vals.pop_front();
//         match l.vals.pop_front() {
//             Some(Expr::Symbol(s)) =>
//                 if let Some(body) = l.vals.pop_front() {
//                     if l.vals.is_empty() {
//                         Ok(Expr::Fun(Fun::new(Box::new(s), Box::new(body), meta)))
//                     } else {
//                         Err(EvalError::ExtraArguments(list))
//                     }
//                 } else {
//                     Err(EvalError::MissingArguments(Expr::List(list), 1))
//                 },
//             Some(other) => Err(EvalError::BadParameter("parameter", other)),
//             None => Err(EvalError::MissingArguments(Expr::List(list), 2)),
//         }
//     }

//     fn eval_the(&mut self, list: List) -> Evaled {
//         let mut l = list.clone();
//         l.vals.pop_front();
//         if let Some(typ) = l.vals.pop_front() {
//             if let Some(val) = l.vals.pop_front() {
//                 if l.vals.is_empty() {
//                     let _typ = self.eval(typ)?;
//                     let val = self.eval(val)?;
//                     Ok(val)
//                 } else {
//                     Err(EvalError::ExtraArguments(list))
//                 }
//             } else {
//                 Err(EvalError::MissingArguments(Expr::List(list), 1))
//             }
//         } else {
//             Err(EvalError::MissingArguments(Expr::List(list), 2))
//         }
//     }
