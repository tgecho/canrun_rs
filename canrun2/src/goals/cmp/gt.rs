use crate::goals::assert_2;
use crate::goals::Goal;
use crate::Unify;
use crate::Value;

/** Ensure that one value is greater than another.

# Example:
```
use canrun2::{unify, all, LVar, Query};
use canrun2::cmp::gt;

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
pub fn gt<A, AV, B, BV>(a: AV, b: BV) -> impl Goal
where
    A: Unify + PartialOrd<B>,
    B: Unify,
    AV: Into<Value<A>>,
    BV: Into<Value<B>>,
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
        goals.assert_permutations_resolve_to((x, y), vec![(2, 1)]);
    }

    #[test]
    fn fails() {
        let (x, y) = (LVar::new(), LVar::new());
        let goals = goal_vec![unify(x, 1), unify(y, 2), gt(x, y)];
        goals.assert_permutations_resolve_to((x, y), vec![]);
    }
}
