use crate::{Can, CanT, State};

pub trait Unify<T: CanT>: Eq + Clone {
    fn resolve_in(&self, state: &State<T>) -> Can<T>;
    fn unify_with(&self, other: &Self, state: &State<T>) -> Option<State<T>>;
}
