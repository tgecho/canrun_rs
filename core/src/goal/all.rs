use super::Goal;
use crate::domains::Domain;
use crate::state::State;

pub(crate) fn run<'a, D>(state: State<'a, D>, goals: Vec<Goal<'a, D>>) -> Option<State<'a, D>>
where
    D: Domain<'a>,
{
    goals.into_iter().try_fold(state, |s, g| g.apply(s))
}

/// Create a [goal](crate::goal::Goal) that only succeeds if all sub-goals succeed.
///
/// This is essentially an "AND" operation on a vector of goals. The resulting state will be the
/// result of the combining all of the sub-goals.
///
/// If the any goal fails, the rest of the goals will not be attempted.
///
/// # Examples
///
/// Multiple successful goals allow values to flow between vars:
/// ```
/// use canrun::{Goal, all, unify, var};
/// use canrun::domains::example::I32;
///
/// let x = var();
/// let y = var();
/// let goal: Goal<I32> = all![unify(y, x), unify(1, x), unify(y, 1)];
/// let result: Vec<_> = goal.query((x, y)).collect();
/// assert_eq!(result, vec![(1, 1)])
/// ```
///
/// A failing goal will cause the entire goal to fail:
/// ```
/// # use canrun::{Goal, all, unify, var};
/// # use canrun::domains::example::I32;
/// # let x = var();
/// # let y = var();
/// let goal: Goal<I32> = all![unify(2, x), unify(1, x), unify(y, x)];
/// let result: Vec<_> = goal.query(x).collect();
/// assert_eq!(result, vec![]) // Empty result
/// ```
#[macro_export]
macro_rules! all {
    ($($item:expr),* $(,)?) => {
        canrun::goal::Goal::All(vec![$($item),*])
    };
}
pub use all;

#[cfg(test)]
mod tests {
    use super::all;
    use crate as canrun;
    use crate::domains::example::I32;
    use crate::goal::unify::unify;
    use crate::goal::Goal;
    use crate::util;
    use crate::value::var;

    #[test]
    fn succeeds() {
        let x = var();
        let y = var();
        let goal: Goal<I32> = all![unify(y, x), unify(y, 1)];
        let result = util::goal_resolves_to(goal, (x, y));
        assert_eq!(result, vec![(1, 1)]);
    }

    #[test]
    fn fails() {
        let x = var();
        let goal: Goal<I32> = all![unify(x, 5), unify(x, 7)];
        let result = util::goal_resolves_to(goal.clone(), x);
        assert_eq!(result, vec![]);
    }
}
