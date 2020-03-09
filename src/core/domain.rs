use super::lvar::LVar;
use super::state::State;
use super::value::Val;
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
    fn values_as_ref(&self) -> &HashMap<LVar, Val<T>>;
    fn values_as_mut(&mut self) -> &mut HashMap<LVar, Val<T>>;
}

pub trait IntoDomainVal<'a, D: Domain<'a>> {
    fn into_domain_val(self) -> D::Value;
}

#[derive(Debug)]
pub struct Just<T> {
    values: HashMap<LVar, Val<T>>,
}

use std::marker::PhantomData;
#[derive(Debug)]
pub struct JustVal<'a, T: UnifyIn<'a, Just<T>>>(Val<T>, PhantomData<&'a T>);

impl<'a, T: UnifyIn<'a, Just<T>> + 'a> IntoDomainVal<'a, Just<T>> for Val<T> {
    fn into_domain_val(self) -> JustVal<'a, T> {
        JustVal(self, PhantomData)
    }
}

impl<'a, T> Clone for Just<T> {
    fn clone(&self) -> Self {
        Just {
            values: self.values.clone(),
        }
    }
}
impl<'a, T: UnifyIn<'a, Just<T>> + 'a> Clone for JustVal<'a, T> {
    fn clone(&self) -> Self {
        JustVal(self.0.clone(), PhantomData)
    }
}

impl<'a, T: UnifyIn<'a, Just<T>> + 'a> Domain<'a> for Just<T> {
    type Value = JustVal<'a, T>;
    fn new() -> Self {
        Just {
            values: HashMap::new(),
        }
    }
    fn unify_domain_values(
        state: State<'a, Self>,
        a: Self::Value,
        b: Self::Value,
    ) -> Option<State<Self>> {
        match (a, b) {
            (JustVal(a, _), JustVal(b, _)) => state.unify(a, b),
        }
    }
}

impl<'a, T: UnifyIn<'a, Just<T>> + 'a> DomainType<'a, T> for Just<T> {
    fn values_as_ref(&self) -> &HashMap<LVar, Val<T>> {
        &self.values
    }
    fn values_as_mut(&mut self) -> &mut HashMap<LVar, Val<T>> {
        &mut self.values
    }
}

impl<'a, T: PartialEq + Debug + 'a> UnifyIn<'a, Just<T>> for T {
    fn unify_with(&self, other: &Self) -> Unified<'a, Just<T>> {
        if self == other {
            Unified::Success
        } else {
            Unified::Failed
        }
    }
}
