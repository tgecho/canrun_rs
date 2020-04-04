use super::Goal;
use crate::domains::{Domain, IntoDomainVal};
use crate::state::State;
use crate::value::IntoVal;

pub(super) fn run<'a, D>(state: State<'a, D>, a: D::Value, b: D::Value) -> Option<State<'a, D>>
where
    D: Domain<'a>,
{
    D::unify_domain_values(state, a, b)
}

// TODO: Write more documentation about what "unification" actually (practically) means?

/// Ensures that two values are unified with each other.
///
/// If one of the values is an unbound [LVar](crate::value::LVar), it will be bound to the other
/// value. If both values are able to be resolved, they will be compared with
/// [Unify::unify_resolved](crate::unify::Unify#tymethod.unify_resolved). If
/// this unification fails, the goal will fail.
///
/// # Examples
///
/// Unifying a fresh LVar will bind it to the other value:
/// ```
/// use canrun::value::var;
/// use canrun::goal::{Goal, unify};
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
/// # use canrun::value::var;
/// # use canrun::goal::{Goal, unify};
/// # use canrun::domains::example::I32;
/// # let x = var();
/// let goal: Goal<I32> = unify(1, 2);
/// let result: Vec<_> = goal.query(x).collect();
/// assert_eq!(result, vec![])
/// ```
pub fn unify<'a, T, A, B, D>(a: A, b: B) -> Goal<'a, D>
where
    D: Domain<'a> + IntoDomainVal<'a, T>,
    A: IntoVal<T>,
    B: IntoVal<T>,
{
    Goal::Unify(
        D::into_domain_val(a.into_val()),
        D::into_domain_val(b.into_val()),
    )
}
