pub mod lvar;

use lvar::LVar;
use std::fmt;

pub trait CanT: Eq + Clone + fmt::Debug {}
impl<T: Eq + Clone + fmt::Debug> CanT for T {}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum Can<T: CanT> {
    Nil,
    Var(LVar),
    Val(T),
    Pair { l: Box<Can<T>>, r: Box<Can<T>> },
    Vec(Vec<Can<T>>),
    // TODO: This could be a more flexible Fn, but we'll need to sort out the eq/hash/etc... issues
    Contains(Box<Can<T>>),
}

impl<T: CanT> Can<T> {
    pub fn is_resolved(&self) -> bool {
        match self {
            Can::Var(_) => false,
            _ => true,
        }
    }
}

impl<T: CanT> From<LVar> for Can<T> {
    fn from(lvar: LVar) -> Self {
        Can::Var(lvar)
    }
}
