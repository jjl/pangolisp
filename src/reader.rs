use crate::forms::*;
use crate::tokens::*;
use crate::exprs::*;
use im::Vector;

pub enum ReadError<'a> {
    UnbalancedMap(Group<Form<'a>>),
}

fn read_all<'a>(forms: Vector<Form<'a>>, exprs: &mut Vector<Expr>)
                -> Result<(), ReadError<'a>> {
    for f in forms {
        exprs.push_back(read(f)?);
    }
    Ok(())
}

pub fn read<'a>(form: Form<'a>) -> Result<Expr, ReadError<'a>> {
    match form {
        Form::Int(int) => {
            let int = Int::new(int.inner, int.span.into());
            Ok(Expr::Int(int))
        }
        Form::Symbol(symbol) => {
            let sym = Symbol::new(symbol.inner.to_string(), symbol.span.into());
            Ok(Expr::Symbol(sym))
        }
        Form::Macro(macr) => {
            let span = macr.span();
            let mut vals = Vector::new();
            match macr {
                Macro::HasType(prefix, typ, val) => {
                    vals.push_back(Expr::Special(Special::The(prefix.span.into())));
                    vals.push_back(read(*typ)?);                    
                    vals.push_back(read(*val)?);
                }
                Macro::Quasiquote(prefix, val) => {
                    vals.push_back(Expr::Special(Special::Quasiquote(prefix.span.into())));
                    vals.push_back(read(*val)?);
                }
                Macro::Quote(prefix, val) => {
                    vals.push_back(Expr::Special(Special::Quote(prefix.span.into())));
                    vals.push_back(read(*val)?);
                }
                Macro::Unquote(prefix, val) => {
                    vals.push_back(Expr::Special(Special::Unquote(prefix.span.into())));
                    vals.push_back(read(*val)?);
                }
            }
            Ok(Expr::List(List::new(vals, span.into())))
        }
        Form::Group(group) => {
            let span = group.span();
            let mut vals = Vector::new();
            match group.open.inner {
                Paren::Paren => {
                    read_all(group.vals, &mut vals)?;
                }
                Paren::Brace => {
                    let sym = Symbol::new("map".to_string(), group.open.span.into());
                    let map = Expr::Symbol(sym);
                    vals.push_back(map);
                    read_all(group.vals, &mut vals)?;
                }
                Paren::Square => {
                    let sym = Symbol::new("list".to_string(), group.open.span.into());
                    let list = Expr::Symbol(sym);
                    vals.push_back(list);
                    read_all(group.vals, &mut vals)?;
                }
            }
            Ok(Expr::List(List::new(vals, span.into())))
        }
    }
}
