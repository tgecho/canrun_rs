pub mod lvar;
pub mod pair;
pub mod vec;

use std::fmt;
use lvar::LVar;
use pair::Pair;

pub trait CanT: Eq + Clone + fmt::Debug {}
impl<T: Eq + Clone + fmt::Debug> CanT for T {}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum Can<T: CanT> {
    Nil,
    Var(LVar),
    Val(T),
    Pair(Pair<T>),
    Vec(Vec<Can<T>>),
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
