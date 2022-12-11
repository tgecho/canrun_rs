use super::unify::Unify;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{any::Any, fmt::Debug, rc::Rc};

pub(crate) type VarId = usize;

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct LVar {
    pub(crate) id: VarId,
}

fn get_id() -> VarId {
    static COUNTER: AtomicUsize = AtomicUsize::new(1);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}
impl LVar {
    fn new() -> Self {
        LVar { id: get_id() }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value<T: Unify> {
    Var(LVar),
    Resolved(Rc<T>),
}

impl<T: Unify> Value<T> {
    pub fn new(t: T) -> Value<T> {
        Value::Resolved(Rc::new(t))
    }
    pub fn var() -> Value<T> {
        Value::Var(LVar::new())
    }

    pub(crate) fn to_anyval(&self) -> AnyVal {
        match self {
            Value::Var(var) => AnyVal::Var(var.id),
            Value::Resolved(val) => AnyVal::Resolved(val.clone()),
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) enum AnyVal {
    Var(VarId),
    Resolved(Rc<dyn Any>),
}

impl AnyVal {
    pub fn to_value<T: Unify>(&self) -> Option<Value<T>> {
        match self {
            AnyVal::Var(id) => Some(Value::Var(LVar { id: *id })),
            AnyVal::Resolved(val) => {
                let rc_t = val.clone().downcast::<T>().ok()?;
                Some(Value::Resolved(rc_t))
            }
        }
    }
}
