mod lvar;

pub(super) use lvar::LVarId;
pub use lvar::{var, LVar};
use std::fmt;
use std::rc::Rc;

pub enum Val<T: ?Sized> {
    Var(LVar<T>),
    Resolved(Rc<T>),
}

use Val::{Resolved, Var};

impl<T> Val<T> {
    // TODO: Need a more ergonomic public .resolve() option
    pub(crate) fn resolved(&self) -> Result<&T, LVar<T>> {
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

pub fn val<T>(t: T) -> Val<T> {
    Val::Resolved(Rc::new(t))
}

impl<T> Clone for Val<T> {
    fn clone(&self) -> Self {
        match self {
            Val::Var(var) => Val::Var(*var),
            Val::Resolved(r) => Val::Resolved(r.clone()),
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
