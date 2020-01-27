pub mod lvar;
pub mod pair;
pub mod vec;

use lvar::LVar;
use pair::Pair;

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
pub enum Can<T: Eq + Clone> {
    Nil,
    Var(LVar),
    Val(T),
    Pair(Pair<T>),
    Vec(Vec<Can<T>>),
}

impl<T: Eq + Clone> From<LVar> for Can<T> {
    fn from(lvar: LVar) -> Self {
        Can::Var(lvar)
    }
}
