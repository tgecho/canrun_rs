use crate::{
    goals::{assert_2, Goal},
    Unify, Value,
};

/** Ensure that one value is greater than or equal to another.

# Example:
```
use canrun::{unify, LVar, all, Query};
use canrun::cmp::gte;

let (x, y, z) = (LVar::new(), LVar::new(), LVar::new());
let goal = all![
    unify(x, 2),
    unify(y, 1),
    unify(z, 1),
    gte(x, y),
    gte(y, z),
];
let results: Vec<_> = goal.query((x, y, z)).collect();
assert_eq!(results, vec![(2, 1, 1)]);
```
*/
pub fn gte<A, B>(a: impl Into<Value<A>>, b: impl Into<Value<B>>) -> impl Goal
where
    A: Unify + PartialOrd<B>,
    B: Unify,
{
    assert_2(a, b, |a, b| a >= b)
}

#[cfg(test)]
mod tests {
    use super::gte;
    use crate::{goal_vec, unify, LVar};

    #[test]
    fn succeeds() {
        let (x, y, z) = (LVar::new(), LVar::new(), LVar::new());
        let goals = goal_vec![unify(x, 2), unify(y, 1), unify(z, 1), gte(x, y), gte(y, z)];
        goals.assert_permutations_resolve_to(&(x, y, z), vec![(2, 1, 1)]);
    }

    #[test]
    fn fails() {
        let (x, y) = (LVar::new(), LVar::new());
        let goals = goal_vec![unify(x, 1), unify(y, 2), gte(x, y)];
        goals.assert_permutations_resolve_to(&(x, y), vec![]);
    }
}
