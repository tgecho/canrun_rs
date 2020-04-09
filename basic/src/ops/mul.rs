use canrun::goal::Goal;
use canrun::map_2;
use canrun::value::IntoVal;
use canrun::Unify;
use std::ops::{Div, Mul};

/// Multiply two values together.
///
/// # Example:
/// ```
/// use canrun::{unify, util, var, Goal};
/// use canrun::domains::example::I32;
/// use canrun_basic::mul;
///
/// let x = var();
/// let goal: Goal<I32> = mul(2, 3, x);
/// let results: Vec<_> = goal.query(x).collect();
/// assert_eq!(results, vec![6]);
/// ```
pub fn mul<'a, T, A, B, C, D>(a: A, b: B, c: C) -> Goal<'a, D>
where
    T: Mul<Output = T> + Div<Output = T> + Copy + 'a,
    A: IntoVal<T>,
    B: IntoVal<T>,
    C: IntoVal<T>,
    D: Unify<'a, T>,
{
    map_2(a, b, c, |a, b| *a * *b, |a, c| *c / *a, |b, c| *c / *b)
}

#[cfg(test)]
mod tests {
    use super::mul;
    use canrun::domains::example::I32;
    use canrun::{unify, util, var, Goal};

    #[test]
    fn succeeds() {
        let (x, y, z) = (var(), var(), var());
        let goals: Vec<Goal<I32>> = vec![unify(x, 2), unify(y, 3), unify(z, 6), mul(x, y, z)];
        util::assert_permutations_resolve_to(goals, (x, y, z), vec![(2, 3, 6)]);
    }

    #[test]
    fn fails() {
        let (x, y, z) = (var(), var(), var());
        let goals: Vec<Goal<I32>> = vec![unify(x, 2), unify(y, 3), unify(z, 5), mul(x, y, z)];
        util::assert_permutations_resolve_to(goals, (x, y, z), vec![]);
    }
}
