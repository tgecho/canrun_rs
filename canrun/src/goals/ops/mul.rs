use crate::{
    goals::{map_2, Goal},
    Unify, Value,
};
use std::ops::{Div, Mul};

/** Multiply two values together.

# Example:
```
use canrun::{unify, LVar, Query};
use canrun::ops::mul;

let x = LVar::new();
let goal = mul(2, 3, x);
let results: Vec<_> = goal.query(x).collect();
assert_eq!(results, vec![6]);
```
*/
pub fn mul<T, A, B, C>(a: A, b: B, c: C) -> impl Goal
where
    T: Unify + Mul<Output = T> + Div<Output = T> + Copy,
    A: Into<Value<T>>,
    B: Into<Value<T>>,
    C: Into<Value<T>>,
{
    map_2(a, b, c, |a, b| *a * *b, |a, c| *c / *a, |b, c| *c / *b)
}

#[cfg(test)]
mod tests {
    use super::mul;
    use crate::{goal_vec, unify, LVar};

    #[test]
    fn succeeds() {
        let (x, y, z) = (LVar::new(), LVar::new(), LVar::new());
        let goals = goal_vec![unify(x, 2), unify(y, 3), unify(z, 6), mul(x, y, z)];
        goals.assert_permutations_resolve_to(&(x, y, z), vec![(2, 3, 6)]);
    }

    #[test]
    fn fails() {
        let (x, y, z) = (LVar::new(), LVar::new(), LVar::new());
        let goals = goal_vec![unify(x, 2), unify(y, 3), unify(z, 5), mul(x, y, z)];
        goals.assert_permutations_resolve_to(&(x, y, z), vec![]);
    }
}
