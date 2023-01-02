use crate::{
    goals::{map_2, Goal},
    Unify, Value,
};
use std::ops::{Add, Sub};

/** Add two values together.

# Example:
```
use canrun::{unify, LVar, Query};
use canrun::ops::add;

let x = LVar::new();
let goal = add(1, 2, x);
let results: Vec<_> = goal.query(x).collect();
assert_eq!(results, vec![3]);
```
*/
pub fn add<T, A, B, C>(a: A, b: B, c: C) -> impl Goal
where
    T: Add<Output = T> + Sub<Output = T> + Unify + Copy,
    A: Into<Value<T>>,
    B: Into<Value<T>>,
    C: Into<Value<T>>,
{
    map_2(a, b, c, |a, b| *a + *b, |a, c| *c - *a, |b, c| *c - *b)
}

#[cfg(test)]
mod tests {
    use super::add;
    use crate::{goal_vec, unify, LVar};

    #[test]
    fn succeeds() {
        let (x, y, z) = (
            LVar::<usize>::new(),
            LVar::<usize>::new(),
            LVar::<usize>::new(),
        );
        let goals = goal_vec![unify(x, 1), unify(y, 2), unify(z, 3), add(x, y, z)];
        goals.assert_permutations_resolve_to((x, y, z), vec![(1, 2, 3)]);
    }

    #[test]
    fn fails() {
        let (x, y, z) = (LVar::new(), LVar::new(), LVar::new());
        let goals = goal_vec![unify(x, 1), unify(y, 2), unify(z, 4), add(x, y, z)];
        goals.assert_permutations_resolve_to((x, y, z), vec![]);
    }
}
