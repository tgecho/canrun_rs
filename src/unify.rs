use crate::cell::Cell;
use crate::state::State;

pub trait Unify<T: Eq + Clone>: Eq + Clone {
    fn resolve_in(&self, state: &State<T>) -> Cell<T>;
    fn unify_with(&self, other: &Self, state: &State<T>) -> Option<State<T>>;
}
