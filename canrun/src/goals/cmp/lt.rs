use crate::{
    goals::{assert_2, Goal},
    Unify, Value,
};

/** Ensure that one value is less than another.

# Example:
```
use canrun::{unify, LVar, all, Query};
use canrun::cmp::lt;

let (x, y) = (LVar::new(), LVar::new());
let goal = all![
    unify(x, 1),
    unify(y, 2),
    lt(x, y)
];
let results: Vec<_> = goal.query((x, y)).collect();
assert_eq!(results, vec![(1, 2)]);
```
*/
pub fn lt<A, B>(a: impl Into<Value<A>>, b: impl Into<Value<B>>) -> impl Goal
where
    A: Unify + PartialOrd<B>,
    B: Unify,
{
    assert_2(a, b, |a, b| a < b)
}

#[cfg(test)]
mod tests {
    use super::lt;
    use crate::{goal_vec, unify, LVar};

    #[test]
    fn succeeds() {
        let (x, y) = (LVar::new(), LVar::new());
        let goals = goal_vec![unify(x, 1), unify(y, 2), lt(x, y)];
        goals.assert_permutations_resolve_to(&(x, y), vec![(1, 2)]);
    }

    #[test]
    fn fails() {
        let (x, y) = (LVar::new(), LVar::new());
        let goals = goal_vec![unify(x, 2), unify(y, 1), lt(x, y)];
        goals.assert_permutations_resolve_to(&(x, y), vec![]);
    }
}
