use crate::unify::Unify;
use crate::{Can, CanT, State};

impl<T: CanT> Unify<T> for Vec<Can<T>> {
    fn resolve_in(&self, state: &State<T>) -> Can<T> {
        let resolved = self.iter().map(|i| state.resolve(i));
        Can::Vec(resolved.collect())
    }

    fn unify_with(&self, other: &Self, state: &State<T>) -> Option<State<T>> {
        if self.len() == other.len() {
            let initial = state.clone();
            let mut pairs = self.iter().zip(other.iter());
            pairs.try_fold(initial, |state, (s, o)| state.unify(s, o))
        } else {
            None
        }
    }
}
