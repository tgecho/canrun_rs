//! Contain individual resolved values or variables that can be bound through
//! [unification](crate::Unify).
mod lvar;

pub(super) use lvar::LVarId;
pub use lvar::{var, LVar};
use std::fmt;
use std::rc::Rc;

/// The possible states a value can be in.
pub enum Val<T: ?Sized> {
    /// A [logical variable](LVar).
    Var(LVar<T>),
    /// A resolved value.
    ///
    /// When a state is split into an arbitrary number of [resolved
    /// states](crate::state::ResolvedState), some of the internal data
    /// structures often need to be cloned. In an attempt to avoid unnecessary
    /// cloning of every value in the state, we wrap it in an [Rc] so that
    /// references can be shared.
    Resolved(Rc<T>),
}

use Val::{Resolved, Var};

impl<T> Val<T> {
    /// Attempt to extract a reference to resolved value (`&T`) or return the
    /// `LVar` if the value is not yet resolved.
    pub fn resolved(&self) -> Result<&T, LVar<T>> {
        match self {
            Resolved(x) => Ok(&*x),
            Var(x) => Err(*x),
        }
    }
}

pub trait IntoVal<T> {
    fn into_val(self) -> Val<T>;
}

impl<T> IntoVal<T> for T {
    fn into_val(self) -> Val<T> {
        Val::Resolved(Rc::new(self))
    }
}

impl<T> IntoVal<T> for Val<T> {
    fn into_val(self) -> Val<T> {
        self
    }
}

impl<T> IntoVal<T> for &Val<T> {
    fn into_val(self) -> Val<T> {
        self.clone()
    }
}

impl<T: Clone> IntoVal<T> for &T {
    fn into_val(self) -> Val<T> {
        Val::Resolved(Rc::new(self.clone()))
    }
}

impl<T> IntoVal<T> for LVar<T> {
    fn into_val(self) -> Val<T> {
        Val::Var(self)
    }
}
impl<T> IntoVal<T> for &LVar<T> {
    fn into_val(self) -> Val<T> {
        Val::Var(self.clone())
    }
}

#[macro_export]
macro_rules! val {
    ($value:expr) => {
        canrun::value::IntoVal::into_val($value)
    };
}

#[doc(inline)]
pub use val;

impl<T> Clone for Val<T> {
    fn clone(&self) -> Self {
        match self {
            Val::Var(var) => Val::Var(*var),
            Val::Resolved(r) => Val::Resolved(r.clone()),
        }
    }
}

impl<T: PartialEq> PartialEq for Val<T> {
    fn eq(&self, other: &Val<T>) -> bool {
        match (self, other) {
            (Resolved(s), Resolved(other)) => s == other,
            (Var(s), Var(other)) => s == other,
            _ => false,
        }
    }
}

impl<'a, T: fmt::Debug> fmt::Debug for Val<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Resolved(v) => write!(f, "Resolved({:?})", v),
            Var(v) => write!(f, "Var({:?})", v),
        }
    }
}
