use super::unify::Unify;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{any::Any, fmt::Debug, rc::Rc};

pub(crate) type LVarId = usize;

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct LVar {
    pub(crate) id: LVarId,
}

fn get_id() -> LVarId {
    static COUNTER: AtomicUsize = AtomicUsize::new(1);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}
impl LVar {
    fn new() -> Self {
        LVar { id: get_id() }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Val<T> {
    Var(LVar),
    Value(Rc<T>),
}

#[derive(Clone, Debug)]
pub(crate) enum AnyVal {
    Var(LVar),
    Value(Rc<dyn Any>),
}
pub fn var<T: Unify>() -> Val<T> {
    Val::Var(LVar::new())
}

pub fn val<T: Unify>(t: T) -> Val<T> {
    Val::Value(Rc::new(t))
}

impl<T: Unify> From<&Val<T>> for AnyVal {
    fn from(value: &Val<T>) -> Self {
        match value {
            Val::Var(var) => AnyVal::Var(*var),
            Val::Value(val) => AnyVal::Value(val.clone()),
        }
    }
}

impl<T: Unify> From<Val<T>> for AnyVal {
    fn from(value: Val<T>) -> Self {
        match value {
            Val::Var(var) => AnyVal::Var(var),
            Val::Value(val) => AnyVal::Value(val),
        }
    }
}
