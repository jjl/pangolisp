use crate::*;
use crate::symbols::*;
use crate::lists::*;
use crate::stack::*;
use std::mem;
use std::iter::DoubleEndedIterator;

use im::HashMap;
// pub enum Kinda {
//     Hole,
//     Star,
//     Arrow,
//     Ref,
// }

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Special {
    Lambda,
    // Match,
    // Quasi,
    Quote,
    // Reset,
    // Shift,
    // The,
    // Unquote,
    // UnquoteSplicing,
}

// pub enum ListBuiltin {
//     Cons,
//     Uncons,
//     Push,
//     Pop,
//     First,
//     Last,
// }

// TODO: floats, ratios etc.
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum Data {
    Nil,
    Bool(bool),
    Int(i64),
}

pub enum Code {
    Data(Data),
    Unresolved(Symbol),
    Symbol(Symbol),
    Special(Special),
    Call(Box<Code>, Box<Code>),
    Lambd(Box<Code>),
    Lambda(Symbol, Box<Code>),
    Quote(Box<Code>),
}

/// A term is our general working data structure
pub enum Term {
    
}

// impl Term {
//     pub fn from_expr(expr: Expr) -> Term {
//     }
// }

pub enum CompileError {
    
}

pub fn desugar(expr: Expr) -> Code {
    match expr {
        Expr::Symbol(sym) => Code::Unresolved(sym),
        Expr::Special(special) => Code::Special(special),
        Expr::Data(data) => Code::Data(data),
        Expr::List(mut list) => {
            match list.values.pop_back() {
                Some(thing) => {
                    let thing = desugar(thing);
                    list.values.into_iter().rfold(thing, |last, val| {
                        let val = desugar(val);
                        Code::Call(Box::new(val), Box::new(last))
                    })
                }
                None => Code::Data(Data::Nil),
            }
        }
    }
}

fn promote_special_calls(expr: Code) -> Result<Code, CompileError> {
    if let Code::Call(a, b) = expr {
        match *a {
            Code::Special(Special::Lambda) => {
                if let Code::Unresolved(symbol) = *b {
                    Ok(Code::Lambd(symbol)),
                } else {
                    Err(CompileError(
                }
            Code::Special(Special::Quote) => Code::Quote(b),
            _ => {
                let b = promote_special_calls(*b);
                match promote_special_calls(*a) {
                    Code::Lambd(a) => Code::Lambda(a, Box::new(b)),
                    other => other,
                }
            }
        }
    } else {
        expr
    }
}

// replace symbols we already know the value of
fn substitute(
    expr: Code,
    subs: &HashMap<Symbol, Code>,
    then: impl FnMut(Code) -> Result<Code, CompileError>
) -> Result<Code, CompileError> {
    match expr {
        Code::Unresolved(sym) => {
            if let Some(val) = subs.get(&sym) {
                *val.clone()
            } else {
                expr
            }
        }
        Code::Call(x, y) => {
            let z = substitute(subs, x.clone(), then)?;
            match z {
                Code::Lambda(param, body) =>
            }
            if x == z {
                
            }
            match x {
                Code::Lambda(
            }
                
            if x == z {
            } else {
                let call = Code::Call(Box::new(x), y);
                let dall = promote_special_calls(call.clone());
                if call == dall {
                    
                } else {
                    Code::Lambda
                    Code::Call(Code::Lambda(x, y)) {
                    }
                }
            }
        }
        Code::
        _ => expr,
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct Param {
    name: Symbol,
    meta: Meta,
}

impl Param {
    #[inline(always)]
    pub fn new(name: Symbol, meta: Meta) -> Param {
        Param { name, meta }
    }
    #[inline(always)]
    pub fn name(&self) -> &Symbol {
        &self.name
    }
    #[inline(always)]
    pub fn meta(&self) -> &Meta {
        &self.meta
    }
}

#[derive(Clone, Eq, Hash)]
pub struct Fun {
    pub param: Param,
    pub body: Box<Expr>,
    pub closure: Vec<(Symbol, Expr)>,
    pub meta: Meta,
}

impl PartialEq for Fun {
    fn eq(&self, other: &Self) -> bool {
        self.param == other.param
            && self.body == other.body
            && self.closure == other.closure
    }
}


#[derive(Clone, Eq, Hash, PartialEq)]
pub enum Expr {
    Symbol(Symbol),
    Special(Special),
    Data(Data),
    List(List<Expr, Meta>),
    // Fun(Fun),
}

#[derive(Clone, Default, Eq, Hash, PartialEq)]
pub struct Meta {
    pub defined_at: Option<Span>,
}

pub enum EvalError {
    ExpectedSymbol(Expr),
    ExpectedCallable(Expr),
    Unbound(Symbol),
    MissingArg,
    ExtraArg(Expr),
}

pub struct Analyser<'a> {
    pub symbols: &'a mut Symbols,
    pub substitutions: Vec<(Symbol, Expr)>,
    pub lexicals: Vec<(Symbol, Expr)>,
}


pub enum AnalysisError {
    Expected(&'static str, Expr),
    SpecialMissingArgs(Special, usize),
    SpecialSurplusArgs(Special, List<Expr, Meta>),
}

// pub fn wat(subs: HashMap<Symbol, Expr>, expr: Expr) -> Result<Wat, AnalysisError> {
//     match expr {
//         Expr::Symbol(sym) => todo!(),
//         Expr::Special(special) => todo!(),
//         Expr::Data(data) => Ok(Wat::Data(data)),
//         Expr::List(mut list) => {
//             match list.values.pop_front() {
//                 Some(Expr::Symbol(sym)) => todo!(),
//                 Some(Expr::Special(special)) => todo!(),
//                 Some(Expr::Data(data)) => todo!(),
//                 Some(Expr::List(list)) => todo!(),
//                 None => Ok(Wat::EmptyList),
//             }
//         }
//     }
// }
// fn sub_lambda(subs: &HashMap<Symbol, Expr>, mut list: List<Expr, Meta>) -> Result<Expr, AnalysisError> {
//     let param = list.values.pop_front().ok_or(AnalysisError::SpecialMissingArgs(Special::Lambda, 2))?;
//     let body = list.values.pop_front().ok_or(AnalysisError::SpecialMissingArgs(Special::Lambda, 1))?;
//     if list.values.is_empty() {
//         match param {
//             Expr::Symbol(sym) => {
//                 // It's not going to be safe to substitute this name as we don't have the value yet
//                 let subs = subs.without(&sym);
//                 let body = substitute(&subs, body)?;
//                 list.values.push_front(body);
//                 list.values.push_front(Expr::Symbol(sym));
//                 list.values.push_front(Expr::Special(Special::Lambda));
//                 Ok(Expr::List(list))
//             }
//             _ => Err(AnalysisError::Expected("Symbol naming parameter", param)),
//         }
//     } else {
//         Err(AnalysisError::SpecialSurplusArgs(Special::Lambda, list))
//     }
// }

// fn sub_list(subs: &HashMap<Symbol, Expr>, head: Expr, List: List<Expr, Meta>) -> Result<Expr, AnalysisError> {
//     match substitute(subs, head) {
//         Expr::Special(special) => {
//             match special {
//                 Special::Lambda => sub_lambda(subs, list),
//                 Special::Quote => {
//                     let arg = list.values.pop_front().ok_or(AnalysisError::SpecialMissingArgs(Special::Quote, 1))?;
//                     if list.values.is_empty() {
//                         list.values.push_front(Expr::Special(Special::Quote));
//                         Ok(Expr::List(list))
//                     } else {
//                         Err(AnalysisError::SpecialSurplusArgs(special, list))
//                     }
//                 }
//             }
//         }
//         Expr::Symbol(sym) => todo!(),
//         Expr::Data(data) => todo!(),
//         Expr::List(sym) => todo!(),
//      }
// }

// // Substitutes known expressions for definite values
// fn substitute(subs: &HashMap<Symbol, Expr>, expr: Expr) -> Result<Expr, AnalysisError> {
//     match expr {
//         Expr::Symbol(sym) => {
//             if let Some(val) = subs.get(&sym) {
//                 Ok(val)
//             } else {
//                 Ok(Expr::Symbol(sym))
//             }
//         }
//         Expr::List(list) => {
//             let mut l = list.clone();
//             match l.values.pop_front() {
//                 Some(front) => sub_list(subs, head, l),
//                 None => Expr::List(list),
//             }
            
//         }
//         _ => Ok(expr)
//     }
// }



// fn wat_special(subs: HashMap<Symbol, Expr>, special: Special, mut list: List<Expr, Meta>) -> Result<Wat, AnalysisError> {
//     match special {
//         Special::Lambda => {
//             let param = list.values.pop_front().ok_or(AnalysisError::SpecialMissingArgs(special, 2))?;
//             let body = list.values.pop_front().ok_or(AnalysisError::SpecialMissingArgs(special, 1))?;
//             if list.values.is_empty() {
//                 match param {
//                     Some(Expr::Symbol(sym)) => {
//                         let tubs = subs.clone();
//                         tubs.remove(&sym);
//                         let body = wat(subs, body)?;
//                         Ok(Wat::Lambda(Param, Box
//                     }
                    
//                 Ok(Wat::Quoted(head))
//             } else {
//                 Err(AnalysisError::SpecialSurplusArgs(special, list))
//             }
//         }
//         Special::Quote => {
//             let head = list.values.pop_front().ok_or(AnalysisError::SpecialMissingArgs(special, 1))?;
//             if list.values.is_empty() {
//                 Ok(Wat::Quoted(head))
//             } else {
//                 Err(AnalysisError::SpecialSurplusArgs(special, list))
//             }
//         }
//     }
// }
    
#[derive(Clone, Eq, PartialEq)]
pub enum Wat {
    Data(Data),
    Quoted(Expr),
    Lambda(Param, Box<Wat>),
    EmptyList,
    List(List<Wat, Meta>),
}

// pub struct Evaluator<'a> {
//     pub symbols: &'a mut Symbols,
//     pub lexicals: Vec<(Symbol, Expr)>,
// }

// impl<'a> Evaluator<'a> {
//     pub fn evaluate(&mut self, expr: Expr) -> Result<Expr, EvalError> {
//         match expr {
//             Expr::Symbol(sym) => {
//                 self.lexicals.iter()
//                     .rfind(|(name,_)| name == &sym)
//                     .map(|(_, expr)| expr.clone())
//                     .ok_or(EvalError::Unbound(sym))
//             }
//             Expr::List(mut l) => {
//                 if let Some(head) = l.values.pop_front() {
//                     let head = self.evaluate(head)?;
//                     match head {
//                         Expr::Special(Special::Lambda) => {
//                             if let Expr::Symbol(s) = l.values.pop_front().ok_or(EvalError::MissingArg)? {
//                                 let param = Param::new(s, Meta::default());
//                                 let body = Box::new(l.values.pop_front().ok_or(EvalError::MissingArg)?);
//                                 let closure = self.lexicals.clone();
//                                 Ok(Expr::Fun(Fun { param, body, closure, meta: Meta::default() }))
//                             } else {
//                                 Err(EvalError::ExpectedSymbol(head))
//                             }
//                         }
//                         Expr::Special(Special::Quote) => {
//                             let val = l.values.pop_front().ok_or(EvalError::MissingArg)?;
//                             if let Some(extra) = l.values.pop_front() {
//                                 Err(EvalError::ExtraArg(extra))
//                             } else {
//                                 Ok(val)
//                             }
//                         }
//                         Expr::Fun(fun) => {
//                             let arg = self.evaluate(l.values.pop_front().ok_or(EvalError::MissingArg)?)?;
//                             let mut closure = fun.closure.clone();
//                             mem::swap(&mut closure, &mut self.lexicals);
//                             self.lexicals.push((fun.param.name, arg));
//                             let body = (*fun.body).clone();
//                             let ret = self.evaluate(body)?;
//                             mem::swap(&mut closure, &mut self.lexicals);
//                             Ok(ret)
//                         }
//                         _ => Err(EvalError::ExpectedCallable(head)),
//                     }
//                 } else {
//                     Ok(Expr::List(l))
//                 }
//             },
//             _ => Ok(expr),
//         }
//     }
// }

// pub enum Analysis {
//     UnboundLexical(Symbol),
//     Data(Data),
//     Free(usize),
//     Fun(Param<Span>),
//     Delayed(Expr<Span>),
// }

// pub fn pass1(expr: Expr) -> Expr {
// }

    //         Expr::List(list) => {
                
    //         }
    //         _ =>
    //         Expr::Fun(fun) => {}
    //     }
    //     // find binding references within body
    //     // match body {
    //     // }
    //     // for each unique reference, if it is outside the frame, copy to closure

    //     // replace all references with de bruijn indices, accounting
    //     // for the closure being pushed to the stack during call
    // }
// }

pub enum Op {
    CallLambda,
    Copy(usize),
}
