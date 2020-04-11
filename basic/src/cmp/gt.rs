use canrun::assert_2;
use canrun::goal::Goal;
use canrun::value::IntoVal;
use canrun::DomainType;

/// Ensure that one value is greater than another.
///
/// # Example:
/// ```
/// use canrun::{unify, util, var, all, Goal};
/// use canrun::domains::example::I32;
/// use canrun_basic::gt;
///
/// let (x, y) = (var(), var());
/// let goal: Goal<I32> = all![
///     unify(x, 2),
///     unify(y, 1),
///     gt(x, y)
/// ];
/// let results: Vec<_> = goal.query((x, y)).collect();
/// assert_eq!(results, vec![(2, 1)]);
/// ```
pub fn gt<'a, A, AV, B, BV, D>(a: AV, b: BV) -> Goal<'a, D>
where
    A: PartialOrd<B> + 'a,
    B: 'a,
    AV: IntoVal<A>,
    BV: IntoVal<B>,
    D: DomainType<'a, A> + DomainType<'a, B>,
{
    assert_2(a, b, |a, b| a > b)
}

#[cfg(test)]
mod tests {
    use super::gt;
    use canrun::domains::example::I32;
    use canrun::{unify, util, var, Goal};

    #[test]
    fn succeeds() {
        let (x, y) = (var(), var());
        let goals: Vec<Goal<I32>> = vec![unify(x, 2), unify(y, 1), gt(x, y)];
        util::assert_permutations_resolve_to(goals, (x, y), vec![(2, 1)]);
    }

    #[test]
    fn fails() {
        let (x, y) = (var(), var());
        let goals: Vec<Goal<I32>> = vec![unify(x, 1), unify(y, 2), gt(x, y)];
        util::assert_permutations_resolve_to(goals, (x, y), vec![]);
    }
}
