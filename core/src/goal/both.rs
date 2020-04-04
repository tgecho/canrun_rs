use super::Goal;
use crate::domains::Domain;
use crate::state::State;

pub(crate) fn run<'a, D>(
    state: State<'a, D>,
    a: Goal<'a, D>,
    b: Goal<'a, D>,
) -> Option<State<'a, D>>
where
    D: Domain<'a>,
{
    a.apply(state).and_then(|s| b.apply(s))
}

/// Create a [Goal](crate::goal::Goal) that only succeeds if both sub-goals succeed.
///
/// This is essentially an "AND" operation. The resulting state will be the
/// result of the combining the two sub-goals.
///
/// If the first goal fails, the second goal will not be attempted.
///
/// # Examples
///
/// Two successful goals allow values to flow between vars:
/// ```
/// use canrun::value::var;
/// use canrun::goal::{Goal, both, unify};
/// use canrun::domains::example::I32;
///
/// let x = var();
/// let y = var();
/// let goal: Goal<I32> = both(unify(y, x), unify(1, x));
/// let result: Vec<_> = goal.query(x).collect();
/// assert_eq!(result, vec![1])
/// ```
///
/// A failing goal will cause the entire goal to fail:
/// ```
/// # use canrun::value::var;
/// # use canrun::goal::{Goal, both, unify};
/// # use canrun::domains::example::I32;
/// # let x = var();
/// let goal: Goal<I32> = both(unify(2, x), unify(1, x));
/// let result: Vec<_> = goal.query(x).collect();
/// assert_eq!(result, vec![]) // Empty result
/// ```
pub fn both<'a, D>(a: Goal<'a, D>, b: Goal<'a, D>) -> Goal<'a, D>
where
    D: Domain<'a>,
{
    Goal::Both(Box::new(a), Box::new(b))
}
