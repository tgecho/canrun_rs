use crate::can::Can;
use crate::state::State;

pub trait Unify<T: Eq + Clone>: Eq + Clone {
    fn resolve_in(&self, state: &State<T>) -> Can<T>;
    fn unify_with(&self, other: &Self, state: &State<T>) -> Option<State<T>>;
}
