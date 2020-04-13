use crate::domains::{Domain, DomainType};
use crate::goal::{Goal, GoalEnum};
use crate::state::State;
use crate::value::IntoVal;
use crate::UnifyIn;
use std::fmt::Debug;

pub(super) fn run<'a, D>(state: State<'a, D>, a: D::Value, b: D::Value) -> Option<State<'a, D>>
where
    D: Domain<'a>,
{
    D::unify_domain_values(state, a, b)
}

/// Create a [goal](crate::goal::Goal) that attempts to [unify](module@crate::unify) two values with each other.
///
/// If one of the values is an unbound [`LVar`](crate::value::LVar), it will be bound to the other
/// value. If both values are able to be resolved, they will be compared with
/// [`UnifyIn::unify_resolved`](crate::unify::UnifyIn#tymethod.unify_resolved). If
/// this unification fails, the goal will fail.
///
/// # Examples
///
/// Unifying a fresh `LVar` will bind it to the other value:
/// ```
/// use canrun::{Goal, unify, var};
/// use canrun::domains::example::I32;
///
/// let x = var();
/// let goal: Goal<I32> = unify(1, x);
/// let result: Vec<_> = goal.query(x).collect();
/// assert_eq!(result, vec![1])
/// ```
///
/// Attempting to unify two unequal values will fail:
/// ```
/// # use canrun::{Goal, unify, var};
/// # use canrun::domains::example::I32;
/// # let x = var();
/// let goal: Goal<I32> = unify(1, 2);
/// let result: Vec<_> = goal.query(x).collect();
/// assert_eq!(result, vec![])
/// ```
pub fn unify<'a, T, A, B, D>(a: A, b: B) -> Goal<'a, D>
where
    T: UnifyIn<'a, D> + Debug,
    A: IntoVal<T>,
    B: IntoVal<T>,
    D: DomainType<'a, T>,
{
    Goal(GoalEnum::UnifyIn(
        D::into_domain_val(a.into_val()),
        D::into_domain_val(b.into_val()),
    ))
}
