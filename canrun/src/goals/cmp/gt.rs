use crate::goals::assert_2;
use crate::goals::Goal;
use crate::Unify;
use crate::Value;

/** Ensure that one value is greater than another.

# Example:
```
use canrun::{unify, all, LVar, Query};
use canrun::cmp::gt;

let (x, y) = (LVar::new(), LVar::new());
let goal = all![
    unify(x, 2),
    unify(y, 1),
    gt(x, y)
];
let results: Vec<_> = goal.query((x, y)).collect();
assert_eq!(results, vec![(2, 1)]);
```
*/
pub fn gt<A, B>(a: impl Into<Value<A>>, b: impl Into<Value<B>>) -> impl Goal
where
    A: Unify + PartialOrd<B>,
    B: Unify,
{
    assert_2(a, b, |a, b| a > b)
}

#[cfg(test)]
mod tests {
    use super::gt;
    use crate::{goal_vec, unify, LVar};

    #[test]
    fn succeeds() {
        let (x, y) = (LVar::new(), LVar::new());
        let goals = goal_vec![unify(x, 2), unify(y, 1), gt(x, y)];
        goals.assert_permutations_resolve_to(&(x, y), vec![(2, 1)]);
    }

    #[test]
    fn fails() {
        let (x, y) = (LVar::new(), LVar::new());
        let goals = goal_vec![unify(x, 1), unify(y, 2), gt(x, y)];
        goals.assert_permutations_resolve_to(&(x, y), vec![]);
    }
}
