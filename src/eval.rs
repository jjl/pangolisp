// use crate::forms::*;
// use crate::spans::*;
use crate::exprs::*;
use im::{HashMap, Vector};

pub enum EvalError {
    BadParameter(&'static str, Expr),
    ExtraArguments(List),
    MissingArguments(Expr, usize),
    NotCallable(Expr, List),
    StackUnderflow(Stack),
    UnknownBinding(String),
    UnexpandedMacro(Fun),
    Reset(Stack),
}

#[derive(Clone, Eq, PartialEq)]
pub struct Eval {
    pub(crate) stack: Stack,
}

impl Eval {

    fn eval_macro(&mut self, m: Fun, mut list: List) -> Result<Expr, EvalError> {
        list.vals.pop_front();
        m.call(Expr::List(list), self)
    }

    pub fn expand_once(&mut self, expr: impl Into<Expr>) -> Result<Expr, EvalError> {
        let expr = expr.into();
        if let Expr::List(list) = expr {
            match list.vals.front() {
                Some(Expr::Macro(m)) => self.eval_macro(m.clone(), list),
                Some(Expr::Symbol(s)) => {
                    if let Ok(Expr::Macro(m)) = self.stack.lookup(&s.value) {
                        let m = m.clone();
                        self.eval_macro(m, list)
                    } else {
                        Ok(Expr::List(list))
                    }
                }
                _ => Ok(Expr::List(list)),
            }
        } else {
            Ok(expr)
        }
    }

    pub fn expand(&mut self, expr: Expr) -> Result<Expr, EvalError> {
        loop {
            let exp = self.expand_once(expr.clone())?;
            if expr == exp {
                return Ok(exp);
            }
        }
    }

    // by this time, symbol resolution and special handling have already occured.
    fn eval_call(&mut self, head: Expr, list: List) -> Result<Expr, EvalError> {
        let mut l = list.vals.clone();
        l.pop_front();
        l.into_iter().try_fold(head, |callable, arg| {
            if let Expr::Fun(f) = callable {
                f.call(self.eval(arg)?, self)
             } else {
                Err(EvalError::NotCallable(callable, list.clone()))
            }
        })
    }

    fn eval_lambda(&mut self, meta: Meta, list: List) -> Result<Expr, EvalError> {
        let mut l = list.clone();
        l.vals.pop_front();
        match l.vals.pop_front() {
            Some(Expr::Symbol(s)) =>
                if let Some(body) = l.vals.pop_front() {
                    if l.vals.is_empty() {
                        Ok(Expr::Fun(Fun::new(Box::new(s), Box::new(body), meta)))
                    } else {
                        Err(EvalError::ExtraArguments(list))
                    }
                } else {
                    Err(EvalError::MissingArguments(Expr::List(list), 1))
                },
            Some(other) => Err(EvalError::BadParameter("parameter", other)),
            None => Err(EvalError::MissingArguments(Expr::List(list), 2)),
        }
    }

    fn eval_quasiquote(&mut self, list: List) -> Result<Expr, EvalError> {
        // TODO
        self.eval_quote(list)
    }

    fn eval_quote(&mut self, list: List) -> Result<Expr, EvalError> {
        let mut l = list.clone();
        l.vals.pop_front();
        if let Some(val) = l.vals.pop_front() {
            if l.vals.is_empty() {
                Ok(val)
            } else {
                Err(EvalError::ExtraArguments(list))
            }
        } else {
            Err(EvalError::MissingArguments(Expr::List(list), 1))
        }
    }

    fn eval_the(&mut self, list: List) -> Result<Expr, EvalError> {
        let mut l = list.clone();
        l.vals.pop_front();
        if let Some(typ) = l.vals.pop_front() {
            if let Some(val) = l.vals.pop_front() {
                if l.vals.is_empty() {
                    let _typ = self.eval(typ)?;
                    let val = self.eval(val)?;
                    Ok(val)
                } else {
                    Err(EvalError::ExtraArguments(list))
                }
            } else {
                Err(EvalError::MissingArguments(Expr::List(list), 1))
            }
        } else {
            Err(EvalError::MissingArguments(Expr::List(list), 2))
        }
    }

    fn eval_unquote(&mut self, _list: List) -> Result<Expr, EvalError> {
        unimplemented!();
    }
 
    fn eval_special_call(&mut self, s: Special, list: List) -> Result<Expr, EvalError> {
        match s {
            Special::Lambda(meta) => self.eval_lambda(meta, list),
            // Special::Match(meta) => { unimplemented!(); }
            Special::Quasiquote(_) => self.eval_quasiquote(list),
            Special::Quote(_) => self.eval_quote(list),
            Special::The(_) => self.eval_the(list),
            Special::Unquote(_) => self.eval_unquote(list),
        }
    }

    fn eval_list_sym(&mut self, expr: Expr, mut list: List) -> Result<Expr, EvalError> {
        if let Expr::Special(s) = expr {
            self.eval_special_call(s, list) 
        } else {
            list.vals.pop_front();
            self.eval_call(expr, list)
        }
    }

    fn eval_list(&mut self, mut list: List) -> Result<Expr, EvalError> {
        match list.vals.front() {
            // for special forms, we do not perturb the expression at
            // all, as if we were executing a compiler macro. We
            // haven't needed this so far, we might relax it later
            Some(Expr::Special(s)) => self.eval_special_call(s.clone(), list),
            Some(Expr::Symbol(s)) => {
                let val = self.stack.lookup(&s.value)?.clone();
                self.eval_list_sym(val, list)
            }
            Some(other) => {
                let other = other.clone();
                list.vals.pop_front();
                self.eval_call(other, list)
            }
            // the empty list is data
            None => Ok(Expr::List(list))
        }
    }

    pub fn eval(&mut self, expr: impl Into<Expr>) -> Result<Expr, EvalError> {
        let expr = self.expand(expr.into())?;
        match expr {
            Expr::Nil => Ok(expr),
            Expr::Int(_) => Ok(expr),
            // Expr::Float(_) => Ok(expr),
            // Expr::String(string) => Ok(expr),
            Expr::Symbol(sym) => self.stack.lookup(&sym.value).map(|e| e.clone()),
            Expr::Special(_) => Ok(expr),
            Expr::List(list) => self.eval_list(list),
            Expr::Map(_) => Ok(expr),
            Expr::Fun(_) => Ok(expr),
            Expr::Macro(m) => Err(EvalError::UnexpandedMacro(m)),
         }
    }
}

#[derive(Clone, Default, Eq, Hash, PartialEq)]
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
    pub fn lookup(&self, name: &str) -> Result<&Expr, EvalError> {
        self.current.get(name).ok_or_else(|| EvalError::UnknownBinding(name.to_string()))
    }
}
