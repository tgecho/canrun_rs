use super::GoalEnum;
use crate::domains::Domain;
use crate::state::{Fork, State};
use std::iter::repeat;

#[derive(Debug)]
pub(super) struct Any<'a, D>
where
    D: Domain<'a>,
{
    pub(super) goals: Vec<GoalEnum<'a, D>>,
}

impl<'a, D> Fork<'a, D> for Any<'a, D>
where
    D: Domain<'a>,
{
    fn fork(&self, state: State<'a, D>) -> crate::state::StateIter<'a, D> {
        let goals = self.goals.clone().into_iter();
        let states = repeat(state);
        Box::new(goals.zip(states).flat_map(|(g, s)| g.apply(s).into_iter()))
    }
}

/// Create a [goal](crate::goal::Goal) that yields a state for every successful
/// sub-goal.
///
/// This is essentially an "OR" operation on a vector of goals. It may yield
/// from zero to as many [resolved states](crate::state::ResolvedState) as there
/// are sub-goals.
///
/// # Examples
///
/// Each successful goal will yield a different result:
/// ```
/// use canrun::{Goal, any, unify, var};
/// use canrun::domains::example::I32;
///
/// let x = var();
/// let goal: Goal<I32> = any![unify(x, 1), unify(x, 2), unify(x, 3)];
/// let result: Vec<_> = goal.query(x).collect();
/// assert_eq!(result, vec![1, 2, 3])
/// ```
///
/// One failing goal will not cause the other to fail:
/// ```
/// # use canrun::{Goal, any, unify, var};
/// # use canrun::domains::example::I32;
/// # let x = var();
/// let goal: Goal<I32> = any!(unify(1, 2), unify(x, 2), unify(x, 3));
/// let result: Vec<_> = goal.query(x).collect();
/// assert_eq!(result, vec![2, 3])
/// ```
///
/// All goals can fail, leading to no results:
/// ```
/// # use canrun::{Goal, any, unify, var};
/// # use canrun::domains::example::I32;
/// # let x = var();
/// let goal: Goal<I32> = any!(unify(6, 5), unify(42, 0), unify(1, 2));
/// let result: Vec<_> = goal.query(x).collect();
/// assert_eq!(result, vec![]) // Empty result
/// ```
#[macro_export]
macro_rules! any {
    ($($item:expr),* $(,)?) => {
        canrun::Goal::any(vec![$($item),*])
    };
}
pub use any;

#[cfg(test)]
mod tests {
    use super::any;
    use crate as canrun;
    use crate::domains::example::I32;
    use crate::goal::unify::unify;
    use crate::goal::Goal;
    use crate::util;
    use crate::value::var;

    #[test]
    fn both_succeeds() {
        let x = var();
        let goal: Goal<I32> = any![unify(x, 5), unify(x, 7)];
        let results = util::goal_resolves_to(goal, x);
        assert_eq!(results, vec![5, 7]);
    }

    #[test]
    fn one_succeeds() {
        let x = var();
        let bad: Goal<I32> = unify(6, 5);

        let first = util::goal_resolves_to(any![unify(x, 1), bad.clone()], x);
        assert_eq!(first, vec![1]);

        let second = util::goal_resolves_to(any![bad, unify(x, 2)], x);
        assert_eq!(second, vec![2]);
    }

    #[test]
    fn both_fail() {
        let x = var();
        let goal: Goal<I32> = any![unify(6, 5), unify(1, 2)];
        let results = util::goal_resolves_to(goal, x);
        assert_eq!(results, vec![]);
    }
}
