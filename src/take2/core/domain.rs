use super::state::State;
use super::val::Val;
use crate::LVar;
use im::HashMap;
use std::fmt;
use std::fmt::Debug;
use std::rc::Rc;

pub enum Unified<'a, D: Domain<'a>> {
    Success,
    Failed,
    Conditional(Box<dyn Fn(State<D>) -> Option<State<D>> + 'a>),
}
pub trait Unify: Debug {
    fn unify_with<'a, D: Domain<'a>>(&self, other: &Self) -> Unified<'a, D>;
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

pub(crate) trait IntoDomainVal<'a, D: Domain<'a>> {
    fn into_domain_val(self) -> D::Value;
}

#[derive(Debug)]
pub struct Just<T> {
    values: HashMap<LVar, Val<T>>,
}

#[derive(Debug)]
pub struct JustVal<T: Unify>(Val<T>);

impl<'a, T: Unify + 'a> IntoDomainVal<'a, Just<T>> for Val<T> {
    fn into_domain_val(self) -> JustVal<T> {
        JustVal(self)
    }
}

impl<'a, T> Clone for Just<T> {
    fn clone(&self) -> Self {
        Just {
            values: self.values.clone(),
        }
    }
}
impl<'a, T: Unify> Clone for JustVal<T> {
    fn clone(&self) -> Self {
        JustVal(self.0.clone())
    }
}

impl<'a, T: Unify + 'a> Domain<'a> for Just<T> {
    type Value = JustVal<T>;
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
            (JustVal(a), JustVal(b)) => state.unify(a, b),
        }
    }
}

impl<'a, T: Unify + 'a> DomainType<'a, T> for Just<T> {
    fn values_as_ref(&self) -> &HashMap<LVar, Val<T>> {
        &self.values
    }
    fn values_as_mut(&mut self) -> &mut HashMap<LVar, Val<T>> {
        &mut self.values
    }
}

impl<T: PartialEq + Debug> Unify for T {
    fn unify_with<'a, D: Domain<'a>>(&self, other: &Self) -> Unified<'a, D> {
        if self == other {
            Unified::Success
        } else {
            Unified::Failed
        }
    }
}
