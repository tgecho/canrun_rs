use crate::unify::Unify;
use crate::{Can, CanT, State};

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct Pair<T: CanT>(Box<(Can<T>, Can<T>)>);

impl<T: CanT> Pair<T> {
    pub fn new(a: Can<T>, b: Can<T>) -> Can<T> {
        Can::Pair(Pair(Box::new((a, b))))
    }
}

impl<T: CanT> Unify<T> for Pair<T> {
    fn resolve_in(&self, state: &State<T>) -> Can<T> {
        let (l, r) = &*self.0;
        Pair::new(state.resolve(l), state.resolve(r))
    }

    fn unify_with(&self, other: &Self, state: &State<T>) -> Option<State<T>> {
        let (a0, a1) = &*self.0;
        let (b0, b1) = &*other.0;
        state.unify(&a0, &b0)?.unify(a1, b1)
    }
}
