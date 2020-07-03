use crate::assert_2;
use crate::goal::Goal;
use crate::value::IntoVal;
use crate::DomainType;
use std::fmt::Debug;

/// Ensure that one value is less than another.
///
/// # Example:
/// ```
/// use canrun::{unify, util, var, all, Goal};
/// use canrun::example::I32;
/// use canrun::lt;
///
/// let (x, y) = (var(), var());
/// let goal: Goal<I32> = all![
///     unify(x, 1),
///     unify(y, 2),
///     lt(x, y)
/// ];
/// let results: Vec<_> = goal.query((x, y)).collect();
/// assert_eq!(results, vec![(1, 2)]);
/// ```
pub fn lt<'a, A, AV, B, BV, D>(a: AV, b: BV) -> Goal<'a, D>
where
    A: PartialOrd<B> + Debug + 'a,
    B: Debug + 'a,
    AV: IntoVal<A>,
    BV: IntoVal<B>,
    D: DomainType<'a, A> + DomainType<'a, B>,
{
    assert_2(a, b, |a, b| a < b)
}

#[cfg(test)]
mod tests {
    use super::lt;
    use crate::example::I32;
    use crate::{unify, util, var, Goal};

    #[test]
    fn succeeds() {
        let (x, y) = (var(), var());
        let goals: Vec<Goal<I32>> = vec![unify(x, 1), unify(y, 2), lt(x, y)];
        util::assert_permutations_resolve_to(goals, (x, y), vec![(1, 2)]);
    }

    #[test]
    fn fails() {
        let (x, y) = (var(), var());
        let goals: Vec<Goal<I32>> = vec![unify(x, 2), unify(y, 1), lt(x, y)];
        util::assert_permutations_resolve_to(goals, (x, y), vec![]);
    }
}
