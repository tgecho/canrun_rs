use crate::{
    goals::{map_2, Goal},
    Unify, Value,
};
use std::ops::{Add, Sub};

/** Subtract one value from another.

# Example:
```
use canrun::{LVar, Query};
use canrun::ops::sub;

let x = LVar::new();
let goal = sub(3, 2, x);
let results: Vec<_> = goal.query(x).collect();
assert_eq!(results, vec![1]);
```
*/
pub fn sub<T, A, B, C>(a: A, b: B, c: C) -> impl Goal
where
    T: Unify + Add<Output = T> + Sub<Output = T> + Copy,
    A: Into<Value<T>>,
    B: Into<Value<T>>,
    C: Into<Value<T>>,
{
    map_2(a, b, c, |a, b| *a - *b, |a, c| *a - *c, |b, c| *b + *c)
}

#[cfg(test)]
mod tests {
    use super::sub;
    use crate::{goal_vec, unify, LVar};

    #[test]
    fn succeeds() {
        let (x, y, z) = (LVar::new(), LVar::new(), LVar::new());
        let goals = goal_vec![unify(x, 3), unify(y, 2), unify(z, 1), sub(x, y, z)];
        goals.assert_permutations_resolve_to(&(x, y, z), vec![(3, 2, 1)]);
    }

    #[test]
    fn fails() {
        let (x, y, z) = (LVar::new(), LVar::new(), LVar::new());
        let goals = goal_vec![unify(x, 3), unify(y, 2), unify(z, 4), sub(x, y, z)];
        goals.assert_permutations_resolve_to(&(x, y, z), vec![]);
    }
}
