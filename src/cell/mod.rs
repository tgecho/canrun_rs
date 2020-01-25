pub mod lvar;
pub mod pair;
pub mod vec;

use lvar::LVar;
use pair::Pair;

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
pub enum Cell<T: Eq + Clone> {
    Nil,
    Var(LVar),
    Value(T),
    Pair(Pair<T>),
    Vec(Vec<Cell<T>>),
}

impl<T: Eq + Clone> From<LVar> for Cell<T> {
    fn from(lvar: LVar) -> Self {
        Cell::Var(lvar)
    }
}
