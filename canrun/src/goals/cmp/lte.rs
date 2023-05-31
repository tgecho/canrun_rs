use crate::{
    goals::{assert_2, Goal},
    Unify, Value,
};

/** Ensure that one value is less than or equal to another.

# Example:
```
use canrun::{unify, LVar, all, Query};
use canrun::cmp::lte;

let (x, y, z) = (LVar::new(), LVar::new(), LVar::new());
let goal = all![
    unify(x, 1),
    unify(y, 2),
    unify(z, 2),
    lte(x, y),
    lte(y, z),
];
let results: Vec<_> = goal.query((x, y, z)).collect();
assert_eq!(results, vec![(1, 2, 2)]);
```
*/
pub fn lte<A, AV, B, BV>(a: AV, b: BV) -> impl Goal
where
    A: Unify + PartialOrd<B>,
    B: Unify,
    AV: Into<Value<A>>,
    BV: Into<Value<B>>,
{
    assert_2(a, b, |a, b| a <= b)
}

#[cfg(test)]
mod tests {
    use super::lte;
    use crate::{goal_vec, unify, LVar};

    #[test]
    fn succeeds() {
        let (x, y, z) = (LVar::new(), LVar::new(), LVar::new());
        let goals = goal_vec![unify(x, 1), unify(y, 2), unify(z, 2), lte(x, y), lte(y, z)];
        goals.assert_permutations_resolve_to(&(x, y, z), vec![(1, 2, 2)]);
    }

    #[test]
    fn fails() {
        let (x, y) = (LVar::new(), LVar::new());
        let goals = goal_vec![unify(x, 2), unify(y, 1), lte(x, y)];
        goals.assert_permutations_resolve_to(&(x, y), vec![]);
    }
}
