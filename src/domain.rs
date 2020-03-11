pub mod one;
pub mod two;

use crate::state::State;
use crate::value::{LVar, Val};
use im::HashMap;
use std::fmt::Debug;

pub enum Unified<'a, D: Domain<'a>> {
    Success,
    Failed,
    Conditional(Box<dyn Fn(State<D>) -> Option<State<D>> + 'a>),
}
pub trait UnifyIn<'a, D: Domain<'a>>: Debug {
    fn unify_with(&self, other: &Self) -> Unified<'a, D>;
}

pub trait Domain<'a>: Clone + Debug {
    type Value: Debug + Clone + 'a;
    fn new() -> Self;
    fn unify_domain_values(
        state: State<'a, Self>,
        a: Self::Value,
        b: Self::Value,
    ) -> Option<State<'a, Self>>;
}

pub trait DomainType<'a, T>: Domain<'a> {
    fn values_as_ref(&self) -> &HashMap<LVar<T>, Val<T>>;
    fn values_as_mut(&mut self) -> &mut HashMap<LVar<T>, Val<T>>;
}

pub trait IntoDomainVal<'a, T>: Domain<'a> {
    fn into_domain_val(val: Val<T>) -> Self::Value;
}
