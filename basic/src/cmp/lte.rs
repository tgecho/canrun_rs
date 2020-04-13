use canrun::assert_2;
use canrun::goal::Goal;
use canrun::value::IntoVal;
use canrun::DomainType;
use std::fmt::Debug;

/// Ensure that one value is less than or equal to another.
///
/// # Example:
/// ```
/// use canrun::{unify, util, var, all, Goal};
/// use canrun::domains::example::I32;
/// use canrun_basic::lte;
///
/// let (x, y, z) = (var(), var(), var());
/// let goal: Goal<I32> = all![
///     unify(x, 1),
///     unify(y, 2),
///     unify(z, 2),
///     lte(x, y),
///     lte(y, z),
/// ];
/// let results: Vec<_> = goal.query((x, y, z)).collect();
/// assert_eq!(results, vec![(1, 2, 2)]);
/// ```
pub fn lte<'a, A, AV, B, BV, D>(a: AV, b: BV) -> Goal<'a, D>
where
    A: PartialOrd<B> + Debug + 'a,
    B: Debug + 'a,
    AV: IntoVal<A>,
    BV: IntoVal<B>,
    D: DomainType<'a, A> + DomainType<'a, B>,
{
    assert_2(a, b, |a, b| a <= b)
}

#[cfg(test)]
mod tests {
    use super::lte;
    use canrun::domains::example::I32;
    use canrun::{unify, util, var, Goal};

    #[test]
    fn succeeds() {
        let (x, y, z) = (var(), var(), var());
        let goals: Vec<Goal<I32>> =
            vec![unify(x, 1), unify(y, 2), unify(z, 2), lte(x, y), lte(y, z)];
        util::assert_permutations_resolve_to(goals, (x, y, z), vec![(1, 2, 2)]);
    }

    #[test]
    fn fails() {
        let (x, y) = (var(), var());
        let goals: Vec<Goal<I32>> = vec![unify(x, 2), unify(y, 1), lte(x, y)];
        util::assert_permutations_resolve_to(goals, (x, y), vec![]);
    }
}
