use crate::core::Unify;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{any::Any, fmt::Debug, rc::Rc};

pub(crate) type VarId = usize;

#[derive(Debug, Copy)]
pub struct LVar<T> {
    pub(crate) id: VarId,
    t: PhantomData<T>,
}

impl<T> PartialEq for LVar<T> {
    fn eq(&self, other: &LVar<T>) -> bool {
        self.id == other.id
    }
}

fn get_id() -> VarId {
    static COUNTER: AtomicUsize = AtomicUsize::new(1);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}
impl<T: Unify> LVar<T> {
    pub fn new() -> Self {
        LVar {
            id: get_id(),
            t: PhantomData::default(),
        }
    }
}

impl<T: Unify> Default for LVar<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, PartialEq)]
pub enum Value<T: Unify> {
    Var(LVar<T>),
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

    pub fn resolved(&self) -> Option<&T> {
        match self {
            Value::Var(_) => None,
            Value::Resolved(val) => Some(val.as_ref()),
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
            AnyVal::Var(id) => Some(Value::Var(LVar {
                id: *id,
                t: PhantomData::default(),
            })),
            AnyVal::Resolved(val) => {
                let rc_t = val.clone().downcast::<T>().ok()?;
                Some(Value::Resolved(rc_t))
            }
        }
    }
}

impl<T: Unify> From<LVar<T>> for Value<T> {
    fn from(var: LVar<T>) -> Self {
        Value::Var(var)
    }
}

impl<T: Unify> From<&LVar<T>> for Value<T> {
    fn from(var: &LVar<T>) -> Self {
        Value::Var(var.clone())
    }
}

impl<T: Unify> From<&Value<T>> for Value<T> {
    fn from(var: &Value<T>) -> Self {
        var.clone()
    }
}

impl<T: Unify> From<T> for Value<T> {
    fn from(t: T) -> Self {
        Value::Resolved(Rc::new(t))
    }
}

// These manual Clone impls are needed because the derive macro adds a `T:
// Clone` constraint. See
// https://doc.rust-lang.org/std/clone/trait.Clone.html#derivable and
// https://stegosaurusdormant.com/understanding-derive-clone/

impl<T> Clone for LVar<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            t: self.t,
        }
    }
}

impl<T: Unify> Clone for Value<T> {
    fn clone(&self) -> Self {
        match self {
            Self::Var(v) => Self::Var(v.clone()),
            Self::Resolved(v) => Self::Resolved(v.clone()),
        }
    }
}
