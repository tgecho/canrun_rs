use super::{Domain, DomainType, IntoDomainVal, Unified, UnifyIn};
use crate::state::State;
use crate::value::{LVar, Val};
use im::HashMap;
use std::fmt::Debug;

type T1 = i32;
type T2 = Vec<i32>;

#[derive(Debug)]
pub struct OfTwo {
    t1: HashMap<LVar<T1>, Val<T1>>,
    t2: HashMap<LVar<T2>, Val<T2>>,
}

#[derive(Debug)]
pub enum OfTwoVal {
    T1(Val<T1>),
    T2(Val<T2>),
}

impl<'a> IntoDomainVal<'a, T1> for OfTwo {
    fn into_domain_val(val: Val<T1>) -> OfTwoVal {
        OfTwoVal::T1(val)
    }
}

impl<'a> IntoDomainVal<'a, T2> for OfTwo {
    fn into_domain_val(val: Val<T2>) -> OfTwoVal {
        OfTwoVal::T2(val)
    }
}

impl<'a> Clone for OfTwo {
    fn clone(&self) -> Self {
        OfTwo {
            t1: self.t1.clone(),
            t2: self.t2.clone(),
        }
    }
}
impl Clone for OfTwoVal {
    fn clone(&self) -> Self {
        match self {
            OfTwoVal::T1(val) => OfTwoVal::T1(val.clone()),
            OfTwoVal::T2(val) => OfTwoVal::T2(val.clone()),
        }
    }
}

impl<'a> Domain<'a> for OfTwo {
    type Value = OfTwoVal;
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
        match (a, b) {
            (OfTwoVal::T1(a), OfTwoVal::T1(b)) => state.unify::<T1, Val<T1>, Val<T1>>(a, b),
            (OfTwoVal::T2(a), OfTwoVal::T2(b)) => state.unify::<T2, Val<T2>, Val<T2>>(a, b),
            _ => None, // This should only happen if a DomainVal constructor allows two values with different types.
        }
    }
}

impl<'a> DomainType<'a, T1> for OfTwo {
    fn values_as_ref(&self) -> &HashMap<LVar<T1>, Val<T1>> {
        &self.t1
    }
    fn values_as_mut(&mut self) -> &mut HashMap<LVar<T1>, Val<T1>> {
        &mut self.t1
    }
}

impl<'a> DomainType<'a, T2> for OfTwo {
    fn values_as_ref(&self) -> &HashMap<LVar<T2>, Val<T2>> {
        &self.t2
    }
    fn values_as_mut(&mut self) -> &mut HashMap<LVar<T2>, Val<T2>> {
        &mut self.t2
    }
}

impl<'a> UnifyIn<'a, OfTwo> for T1 {
    fn unify_with(&self, other: &Self) -> Unified<'a, OfTwo> {
        if self == other {
            Unified::Success
        } else {
            Unified::Failed
        }
    }
}
impl<'a> UnifyIn<'a, OfTwo> for T2 {
    fn unify_with(&self, other: &Self) -> Unified<'a, OfTwo> {
        if self == other {
            Unified::Success
        } else {
            Unified::Failed
        }
    }
}

#[cfg(test)]
mod tests {
    use super::OfTwo;
    use crate::goal::{all, project, unify, Goal};
    use crate::state::{State, Watch};
    use crate::tests::util;
    use crate::value::{var, Val};

    #[test]
    fn succeeds() {
        let x = var::<Vec<i32>>();
        let y = var::<i32>();
        let goal: Goal<OfTwo> = all::<OfTwo>(vec![
            unify(x, vec![1, 2, 3]),
            unify(y, 1),
            project(|s: State<OfTwo>| {
                // This is pretty gnarly
                let x = Val::Var(x);
                let x = s.resolve_val(&x).resolved();
                let y = Val::Var(y);
                let y = s.resolve_val(&y).resolved();
                match (x, y) {
                    (Ok(x), Ok(y)) => Watch::done(if x.contains(y) { Some(s) } else { None }),
                    (Err(x), Err(y)) => Watch::watch(s, x).and(y),
                    (_, Err(y)) => Watch::watch(s, y),
                    (Err(x), _) => Watch::watch(s, x),
                }
            }) as Goal<OfTwo>,
        ]);
        let result = util::goal_resolves_to(goal, (&x, &y));
        assert_eq!(result, vec![(vec![1, 2, 3], 1)]);
    }
}
