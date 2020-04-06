use super::Goal;
use super::Project;
use crate::domains::DomainType;
use crate::state::State;
use crate::state::Watch;
use crate::value::{
    IntoVal, Val,
    Val::{Resolved, Var},
};
use std::fmt;
use std::rc::Rc;

pub struct Assert1<'a, A> {
    a: Val<A>,
    f: Rc<dyn Fn(&A) -> bool + 'a>,
}

/// Create a [projection goal](super) that succeeds if the resolved value passes
/// an assertion test.
///
/// ```
/// use canrun::{Goal, both, unify, var, assert_1};
/// use canrun::domains::example::I32;
///
/// let x = var();
/// let goal: Goal<I32> = both(unify(1, x), assert_1(x, |x| *x < 2));
/// let result: Vec<_> = goal.query(x).collect();
/// assert_eq!(result, vec![1])
/// ```
pub fn assert_1<'a, A: 'a, AV, D, F>(a: AV, func: F) -> Goal<'a, D>
where
    AV: IntoVal<A>,
    D: DomainType<'a, A>,
    F: Fn(&A) -> bool + 'a,
{
    Goal::Project(Rc::new(Assert1 {
        a: a.into_val(),
        f: Rc::new(func),
    }))
}

impl<'a, A, Dom> Project<'a, Dom> for Assert1<'a, A>
where
    Dom: DomainType<'a, A>,
{
    fn attempt<'r>(&'r self, state: State<'a, Dom>) -> Watch<State<'a, Dom>> {
        let a = state.resolve_val(&self.a).clone();
        match a {
            Resolved(a) => Watch::done({
                let assert = self.f.clone();
                if assert(&*a) {
                    Some(state)
                } else {
                    None
                }
            }),
            Var(var) => Watch::watch(state, var),
        }
    }
}

impl<'a, A> fmt::Debug for Assert1<'a, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Assert1 ??")
    }
}
