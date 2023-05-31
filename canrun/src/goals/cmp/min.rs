use crate::cmp::{lt, lte};
use crate::goals::Goal;
use crate::{both, either, unify, Unify, Value};

/** Get the lesser of two values according to [`std::cmp::min`].

# Example:
```
use canrun::{unify, LVar, all, Query};
use canrun::cmp::min;

let (x, y, z) = (LVar::new(), LVar::new(), LVar::new());
let goal = all![
    unify(x, 1),
    unify(y, 2),
    unify(z, 1),
    min(x, y, z),
];
let results: Vec<_> = goal.query((x, y, z)).collect();
assert_eq!(results, vec![(1, 2, 1)]);
```
*/
pub fn min<T, A, B, C>(a: A, b: B, c: C) -> impl Goal
where
    T: Unify + PartialOrd,
    A: Into<Value<T>>,
    B: Into<Value<T>>,
    C: Into<Value<T>>,
{
    let a = a.into();
    let b = b.into();
    let c = c.into();
    either(
        both(unify(a.clone(), c.clone()), lte(a.clone(), b.clone())),
        // Using lte above and just lt below avoids multiple states when they are equal
        // I'm not 100% sure this will be generally correct
        both(unify(b.clone(), c), lt(b, a)),
    )
}

#[cfg(test)]
mod tests {
    use super::min;
    use crate::{goal_vec, unify, LVar};

    #[test]
    fn succeeds_lt() {
        let (x, y, z) = (LVar::new(), LVar::new(), LVar::new());
        let goals = goal_vec![unify(x, 1), unify(y, 2), unify(z, 1), min(x, y, z)];
        goals.assert_permutations_resolve_to(&(x, y, z), vec![(1, 2, 1)]);
    }
    #[test]
    fn succeeds_lte() {
        let (x, y, z) = (LVar::new(), LVar::new(), LVar::new());
        let goals = goal_vec![unify(x, 1), unify(y, 1), unify(z, 1), min(x, y, z)];
        goals.assert_permutations_resolve_to(&(x, y, z), vec![(1, 1, 1)]);
    }

    #[test]
    fn fails() {
        let (x, y, z) = (LVar::new(), LVar::new(), LVar::new());
        let goals = goal_vec![unify(x, 2), unify(y, 1), unify(z, 2), min(x, y, z)];
        goals.assert_permutations_resolve_to(&(x, y), vec![]);
    }
}
