use crate::unify::Unify;
use crate::{Cell, State};

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct Pair<T: Eq + Clone>(Box<(Cell<T>, Cell<T>)>);

impl<T: Eq + Clone> Pair<T> {
    pub fn new(a: Cell<T>, b: Cell<T>) -> Cell<T> {
        Cell::Pair(Pair(Box::new((a, b))))
    }
}

impl<T: Eq + Clone> Unify<T> for Pair<T> {
    fn resolve_in(&self, state: &State<T>) -> Cell<T> {
        let (l, r) = &*self.0;
        Pair::new(state.resolve(l), state.resolve(r))
    }

    fn unify_with(&self, other: &Self, state: &State<T>) -> Option<State<T>> {
        let (a0, a1) = &*self.0;
        let (b0, b1) = &*other.0;
        state.unify(&a0, &b0)?.unify(a1, b1)
    }
}
