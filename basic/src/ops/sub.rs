use canrun::goal::Goal;
use canrun::map_2;
use canrun::value::IntoVal;
use canrun::Unify;
use std::ops::{Add, Sub};

/// Subtract one value from another.
///
/// # Example:
/// ```
/// use canrun::{unify, util, var, Goal};
/// use canrun::domains::example::I32;
/// use canrun_basic::sub;
///
/// let x = var();
/// let goal: Goal<I32> = sub(3, 2, x);
/// let results: Vec<_> = goal.query(x).collect();
/// assert_eq!(results, vec![1]);
/// ```
pub fn sub<'a, T, A, B, C, D>(a: A, b: B, c: C) -> Goal<'a, D>
where
    T: Add<Output = T> + Sub<Output = T> + Copy + 'a,
    A: IntoVal<T>,
    B: IntoVal<T>,
    C: IntoVal<T>,
    D: Unify<'a, T>,
{
    map_2(a, b, c, |a, b| *a - *b, |a, c| *a - *c, |b, c| *b + *c)
}

#[cfg(test)]
mod tests {
    use super::sub;
    use canrun::domains::example::I32;
    use canrun::{unify, util, var, Goal};

    #[test]
    fn succeeds() {
        let (x, y, z) = (var(), var(), var());
        let goals: Vec<Goal<I32>> = vec![unify(x, 3), unify(y, 2), unify(z, 1), sub(x, y, z)];
        util::assert_permutations_resolve_to(goals, (x, y, z), vec![(3, 2, 1)]);
    }

    #[test]
    fn fails() {
        let (x, y, z) = (var(), var(), var());
        let goals: Vec<Goal<I32>> = vec![unify(x, 3), unify(y, 2), unify(z, 4), sub(x, y, z)];
        util::assert_permutations_resolve_to(goals, (x, y, z), vec![]);
    }
}
