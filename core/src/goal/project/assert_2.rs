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

pub struct Assert2<'a, A, B> {
    a: Val<A>,
    b: Val<B>,
    f: Rc<dyn Fn(&A, &B) -> bool + 'a>,
}

/// Create a [projection goal](super) that succeeds if the resolved values pass
/// an assertion test.
///
/// ```
/// use canrun::{Goal, all, unify, var, assert_2};
/// use canrun::domains::example::I32;
///
/// let (x, y) = (var(), var());
/// let goal: Goal<I32> = all![
///     unify(1, x),
///     unify(2, y),
///     assert_2(x, y, |x, y| x < y),
/// ];
/// let result: Vec<_> = goal.query((x, y)).collect();
/// assert_eq!(result, vec![(1, 2)])
/// ```
pub fn assert_2<'a, A: 'a, AV, B: 'a, BV, D, F>(a: AV, b: BV, func: F) -> Goal<'a, D>
where
    AV: IntoVal<A>,
    BV: IntoVal<B>,
    D: DomainType<'a, A> + DomainType<'a, B>,
    F: Fn(&A, &B) -> bool + 'a,
{
    Goal::Project(Rc::new(Assert2 {
        a: a.into_val(),
        b: b.into_val(),
        f: Rc::new(func),
    }))
}

impl<'a, A, B, Dom> Project<'a, Dom> for Assert2<'a, A, B>
where
    Dom: DomainType<'a, A> + DomainType<'a, B>,
{
    fn attempt<'r>(&'r self, state: State<'a, Dom>) -> Watch<State<'a, Dom>> {
        let a = state.resolve_val(&self.a).clone();
        let b = state.resolve_val(&self.b).clone();
        match (a, b) {
            (Resolved(a), Resolved(b)) => Watch::done({
                let assert = self.f.clone();
                if assert(&*a, &*b) {
                    Some(state)
                } else {
                    None
                }
            }),
            (Var(var), _) => Watch::watch(state, var),
            (_, Var(var)) => Watch::watch(state, var),
        }
    }
}

impl<'a, A, B> fmt::Debug for Assert2<'a, A, B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Assert2 ??")
    }
}
