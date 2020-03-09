mod lvar;

pub use lvar::LVar;
use std::rc::Rc;

#[derive(Debug)]
pub enum Val<T: ?Sized> {
    Var(LVar),
    Resolved(Rc<T>),
}

use Val::{Resolved, Var};

impl<T> Val<T> {
    pub(crate) fn resolved(&self) -> Result<&T, LVar> {
        match self {
            Resolved(x) => Ok(&*x),
            Var(x) => Err(*x),
        }
    }
}

pub fn val<T>(t: T) -> Val<T> {
    Val::Resolved(Rc::new(t))
}

pub fn var<T>() -> Val<T> {
    Val::Var(LVar::new())
}

// I don't actually understand why derive(Clone) doesn't seem to work
// without T: Clone but this seems to work
impl<T> Clone for Val<T> {
    fn clone(&self) -> Self {
        match self {
            Val::Var(var) => Val::Var(*var),
            Val::Resolved(r) => Val::Resolved(r.clone()),
        }
    }
}
