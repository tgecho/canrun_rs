use super::{Domain, DomainType, IntoDomainVal, Unified, UnifyIn};
use crate::core::state::State;
use crate::value::{LVar, Val};
use im::HashMap;
use std::fmt::Debug;

#[derive(Debug)]
pub struct OfOne<T> {
    values: HashMap<LVar, Val<T>>,
}

use std::marker::PhantomData;
#[derive(Debug)]
pub struct OfOneVal<'a, T: UnifyIn<'a, OfOne<T>>>(Val<T>, PhantomData<&'a T>);

impl<'a, T: UnifyIn<'a, OfOne<T>> + 'a> IntoDomainVal<'a, OfOne<T>> for Val<T> {
    fn into_domain_val(self) -> OfOneVal<'a, T> {
        OfOneVal(self, PhantomData)
    }
}

impl<'a, T> Clone for OfOne<T> {
    fn clone(&self) -> Self {
        OfOne {
            values: self.values.clone(),
        }
    }
}
impl<'a, T: UnifyIn<'a, OfOne<T>> + 'a> Clone for OfOneVal<'a, T> {
    fn clone(&self) -> Self {
        OfOneVal(self.0.clone(), PhantomData)
    }
}

impl<'a, T: UnifyIn<'a, OfOne<T>> + 'a> Domain<'a> for OfOne<T> {
    type Value = OfOneVal<'a, T>;
    fn new() -> Self {
        OfOne {
            values: HashMap::new(),
        }
    }
    fn unify_domain_values(
        state: State<'a, Self>,
        a: Self::Value,
        b: Self::Value,
    ) -> Option<State<Self>> {
        match (a, b) {
            (OfOneVal(a, _), OfOneVal(b, _)) => state.unify(a, b),
        }
    }
}

impl<'a, T: UnifyIn<'a, OfOne<T>> + 'a> DomainType<'a, T> for OfOne<T> {
    fn values_as_ref(&self) -> &HashMap<LVar, Val<T>> {
        &self.values
    }
    fn values_as_mut(&mut self) -> &mut HashMap<LVar, Val<T>> {
        &mut self.values
    }
}

impl<'a, T: PartialEq + Debug + 'a> UnifyIn<'a, OfOne<T>> for T {
    fn unify_with(&self, other: &Self) -> Unified<'a, OfOne<T>> {
        if self == other {
            Unified::Success
        } else {
            Unified::Failed
        }
    }
}
