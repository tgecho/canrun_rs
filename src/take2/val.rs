use crate::can::lvar::LVar;
use std::rc::Rc;

#[derive(PartialEq)]
pub enum Val<T: ?Sized> {
    Var(LVar),
    Resolved(Rc<T>),
}

pub fn val<T>(t: T) -> Val<T> {
    Val::Resolved(Rc::new(t))
}

// I don't actually understand why derive(Clone) doesn't seem to work
// without T: Clone but this does
impl<T> Clone for Val<T> {
    fn clone(&self) -> Self {
        match self {
            Val::Var(var) => Val::Var(*var),
            Val::Resolved(r) => Val::Resolved(r.clone()),
        }
    }
}
