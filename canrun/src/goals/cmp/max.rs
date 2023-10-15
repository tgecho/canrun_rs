use crate::cmp::{gt, gte};
use crate::goals::Goal;
use crate::{both, either, unify, Unify, Value};

/** Get the greater of two values according to [`std::cmp::max`].

# Example:
```
use canrun::{unify, all, LVar, Query};
use canrun::cmp::max;

let (x, y, z) = (LVar::new(), LVar::new(), LVar::new());
let goal = all![
    unify(x, 1),
    unify(y, 2),
    unify(z, 2),
    max(x, y, z),
];
let results: Vec<_> = goal.query((x, y, z)).collect();
assert_eq!(results, vec![(1, 2, 2)]);
```
*/
pub fn max<T>(a: impl Into<Value<T>>, b: impl Into<Value<T>>, c: impl Into<Value<T>>) -> impl Goal
where
    T: Unify + PartialOrd,
{
    let a = a.into();
    let b = b.into();
    let c = c.into();
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
    use crate::{goal_vec, unify, LVar};

    #[test]
    fn succeeds_gt() {
        let (x, y, z) = (LVar::new(), LVar::new(), LVar::new());
        let goals = goal_vec![unify(x, 1), unify(y, 2), unify(z, 2), max(x, y, z)];
        goals.assert_permutations_resolve_to(&(x, y, z), vec![(1, 2, 2)]);
    }
    #[test]
    fn succeeds_gte() {
        let (x, y, z) = (LVar::new(), LVar::new(), LVar::new());
        let goals = goal_vec![unify(x, 1), unify(y, 1), unify(z, 1), max(x, y, z)];
        goals.assert_permutations_resolve_to(&(x, y, z), vec![(1, 1, 1)]);
    }

    #[test]
    fn fails() {
        let (x, y, z) = (LVar::new(), LVar::new(), LVar::new());
        let goals = goal_vec![unify(x, 2), unify(y, 1), unify(z, 1), max(x, y, z)];
        goals.assert_permutations_resolve_to(&(x, y), vec![]);
    }
}
