use crate::{gt, gte};
use canrun::goal::Goal;
use canrun::value::IntoVal;
use canrun::{both, either, unify, val};
use canrun::{DomainType, UnifyIn};

/// Get the greater of two values according to [`std::cmp::max`].
///
/// # Example:
/// ```
/// use canrun::{unify, util, var, all, Goal};
/// use canrun::domains::example::I32;
/// use canrun_basic::max;
///
/// let (x, y, z) = (var(), var(), var());
/// let goal: Goal<I32> = all![
///     unify(x, 1),
///     unify(y, 2),
///     unify(z, 2),
///     max(x, y, z),
/// ];
/// let results: Vec<_> = goal.query((x, y, z)).collect();
/// assert_eq!(results, vec![(1, 2, 2)]);
/// ```
pub fn max<'a, T, A, B, C, D>(a: A, b: B, c: C) -> Goal<'a, D>
where
    T: PartialOrd + UnifyIn<'a, D> + 'a,
    A: IntoVal<T>,
    B: IntoVal<T>,
    C: IntoVal<T>,
    D: DomainType<'a, T>,
{
    let a = val!(a);
    let b = val!(b);
    let c = val!(c);
    either(
        both(unify(a.clone(), c.clone()), gte(a.clone(), b.clone())),
        // Using gte above and just gt below avoids multiple states when they are equal
        // I'm not 100% sure this will be generally correct
        both(unify(b.clone(), c), gt(b, a)),
    )
}

#[cfg(test)]
mod tests {
    use super::max;
    use canrun::domains::example::I32;
    use canrun::{unify, util, var, Goal};

    #[test]
    fn succeeds_gt() {
        let (x, y, z) = (var(), var(), var());
        let goals: Vec<Goal<I32>> = vec![unify(x, 1), unify(y, 2), unify(z, 2), max(x, y, z)];
        util::assert_permutations_resolve_to(goals, (x, y, z), vec![(1, 2, 2)]);
    }
    #[test]
    fn succeeds_gte() {
        let (x, y, z) = (var(), var(), var());
        let goals: Vec<Goal<I32>> = vec![unify(x, 1), unify(y, 1), unify(z, 1), max(x, y, z)];
        util::assert_permutations_resolve_to(goals, (x, y, z), vec![(1, 1, 1)]);
    }

    #[test]
    fn fails() {
        let (x, y, z) = (var(), var(), var());
        let goals: Vec<Goal<I32>> = vec![unify(x, 2), unify(y, 1), unify(z, 1), max(x, y, z)];
        util::assert_permutations_resolve_to(goals, (x, y), vec![]);
    }
}
