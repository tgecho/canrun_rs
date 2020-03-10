use super::{Domain, DomainType, IntoDomainVal, Unified, UnifyIn};
use crate::state::State;
use crate::value::{IntoVal, LVar, Val};
use im::HashMap;
use std::fmt::Debug;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct OfTwo<T1, T2> {
    t1: HashMap<LVar<T1>, Val<T1>>,
    t2: HashMap<LVar<T2>, Val<T2>>,
}

#[derive(Debug)]
pub enum OfTwoVal<'a, T1: UnifyIn<'a, OfTwo<T1, T2>>, T2: UnifyIn<'a, OfTwo<T1, T2>>> {
    T1(Val<T1>, PhantomData<&'a T1>),
    T2(Val<T2>, PhantomData<&'a T2>),
}

impl<'a, T1: UnifyIn<'a, OfTwo<T1, T2>>, T2: UnifyIn<'a, OfTwo<T1, T2>> + 'a, V: IntoVal<T1>>
    IntoDomainVal<'a, OfTwo<T1, T2>> for V
{
    fn into_domain_val(self) -> OfTwoVal<'a, T1, T2> {
        OfTwoVal::T1(self.into_val(), PhantomData)
    }
}

impl<'a, T1: UnifyIn<'a, OfTwo<T1, T2>>, T2: UnifyIn<'a, OfTwo<T1, T2>> + 'a, V: IntoVal<T2>>
    IntoDomainVal<'a, OfTwo<T1, T2>> for V
{
    fn into_domain_val(self) -> OfTwoVal<'a, T1, T2> {
        OfTwoVal::T2(self.into_val(), PhantomData)
    }
}

impl<'a, T1, T2> Clone for OfTwo<T1, T2> {
    fn clone(&self) -> Self {
        OfTwo {
            t1: self.t1.clone(),
            t2: self.t2.clone(),
        }
    }
}
impl<'a, T1: UnifyIn<'a, OfTwo<T1, T2>>, T2: UnifyIn<'a, OfTwo<T1, T2>> + 'a> Clone
    for OfTwoVal<'a, T1, T2>
{
    fn clone(&self) -> Self {
        match self {
            OfTwoVal::T1(val) => OfTwoVal::T1(val.clone()),
            OfTwoVal::T2(val) => OfTwoVal::T2(val.clone()),
        }
    }
}

impl<'a, T1: UnifyIn<'a, OfTwo<T1, T2>> + 'a, T2: UnifyIn<'a, OfTwo<T1, T2>> + 'a> Domain<'a>
    for OfTwo<T1, T2>
{
    type Value = OfTwoVal<'a, T1, T2>;
    fn new() -> Self {
        OfTwo {
            t1: HashMap::new(),
            t2: HashMap::new(),
        }
    }
    fn unify_domain_values(
        state: State<'a, Self>,
        a: Self::Value,
        b: Self::Value,
    ) -> Option<State<Self>> {
        use OfTwoVal::*;

        match (a, b) {
            (T1(a), T1(b)) => state.unify(a, b),
        }
    }
}

impl<'a, T1: UnifyIn<'a, OfTwo<T1, T2>> + 'a, T2: UnifyIn<'a, OfTwo<T1, T2>> + 'a>
    DomainType<'a, T1> for OfTwo<T1, T2>
{
    fn values_as_ref(&self) -> &HashMap<LVar<T1>, Val<T1>> {
        &self.t1
    }
    fn values_as_mut(&mut self) -> &mut HashMap<LVar<T1>, Val<T1>> {
        &mut self.t1
    }
}

impl<'a, T1: UnifyIn<'a, OfTwo<T1, T2>> + 'a, T2: UnifyIn<'a, OfTwo<T1, T2>> + 'a>
    DomainType<'a, T2> for OfTwo<T1, T2>
{
    fn values_as_ref(&self) -> &HashMap<LVar<T2>, Val<T2>> {
        &self.t2
    }
    fn values_as_mut(&mut self) -> &mut HashMap<LVar<T2>, Val<T2>> {
        &mut self.t2
    }
}

impl<'a, T1: PartialEq + Debug + 'a, T2: PartialEq + Debug + 'a> UnifyIn<'a, OfTwo<T1, T2>> for T1 {
    fn unify_with(&self, other: &Self) -> Unified<'a, OfTwo<T1, T2>> {
        if self == other {
            Unified::Success
        } else {
            Unified::Failed
        }
    }
}
