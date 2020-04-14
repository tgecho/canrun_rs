use super::{Goal, GoalEnum};
use crate::domains::Domain;
use crate::state::{Fork, State};

#[derive(Debug)]
struct Either<'a, D>
where
    D: Domain<'a>,
{
    a: GoalEnum<'a, D>,
    b: GoalEnum<'a, D>,
}

impl<'a, D> Fork<'a, D> for Either<'a, D>
where
    D: Domain<'a>,
{
    fn fork(&self, state: State<'a, D>) -> crate::state::StateIter<'a, D> {
        let a = self.a.clone().apply(state.clone()).into_iter();
        let b = self.b.clone().apply(state).into_iter();
        Box::new(a.chain(b))
    }
}

/// Create a [goal](crate::goal::Goal) that succeeds if either sub-goal succeed.
///
/// This is essentially an "OR" operation, and will eventually lead to zero, one
/// or two [resolved states](crate::state::ResolvedState), depending on the success
/// or failure of the sub-goals.
///
/// # Examples
///
/// Two successful goals will yield up two different results:
/// ```
/// use canrun::value::var;
/// use canrun::goal::{Goal, either, unify};
/// use canrun::domains::example::I32;
///
/// let x = var();
/// let goal: Goal<I32> = either(unify(x, 1), unify(x, 2));
/// let result: Vec<_> = goal.query(x).collect();
/// assert_eq!(result, vec![1, 2])
/// ```
///
/// One failing goal will not cause the other to fail:
/// ```
/// # use canrun::{Goal, either, unify, var};
/// # use canrun::domains::example::I32;
/// # let x = var();
/// let goal: Goal<I32> = either(unify(1, 2), unify(x, 3));
/// let result: Vec<_> = goal.query(x).collect();
/// assert_eq!(result, vec![3])
/// ```
///
/// Both goals can fail, leading to no results:
/// ```
/// # use canrun::{Goal, either, unify, var};
/// # use canrun::domains::example::I32;
/// # let x = var();
/// let goal: Goal<I32> = either(unify(6, 5), unify(1, 2));
/// let result: Vec<_> = goal.query(x).collect();
/// assert_eq!(result, vec![]) // Empty result
/// ```
pub fn either<'a, D>(a: Goal<'a, D>, b: Goal<'a, D>) -> Goal<'a, D>
where
    D: Domain<'a>,
{
    Goal::fork(Either { a: a.0, b: b.0 })
}

#[cfg(test)]
mod tests {
    use super::either;
    use crate::domains::example::I32;
    use crate::goal::unify::unify;
    use crate::goal::Goal;
    use crate::util;
    use crate::value::var;

    #[test]
    fn either_both_succeeds() {
        let x = var();
        let goal = either::<I32>(unify(x, 5), unify(x, 7));
        let results = util::goal_resolves_to(goal, x);
        assert_eq!(results, vec![5, 7]);
    }

    #[test]
    fn either_one_succeeds() {
        let x = var();
        let bad: Goal<I32> = unify(6, 5);

        let first = util::goal_resolves_to(either(unify(x, 1), bad.clone()), x);
        assert_eq!(first, vec![1]);

        let second = util::goal_resolves_to(either(bad, unify(x, 2)), x);
        assert_eq!(second, vec![2]);
    }

    #[test]
    fn either_both_fail() {
        let x = var();
        let goal: Goal<I32> = either(unify(6, 5), unify(1, 2));
        let results = util::goal_resolves_to(goal, x);
        assert_eq!(results, vec![]);
    }
}
