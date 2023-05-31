use crate::{
    goals::{map_2, Goal},
    Unify, Value,
};
use std::ops::{Div, Mul};

/** Divide one value with another.

# Example:
```
use canrun::{LVar, Query};
use canrun::ops::div;

let x = LVar::new();
let goal = div(3, 2, x);
let results: Vec<_> = goal.query(x).collect();
assert_eq!(results, vec![1]);
```
*/
pub fn div<T, A, B, C>(a: A, b: B, c: C) -> impl Goal
where
    T: Unify + Mul<Output = T> + Div<Output = T> + Copy,
    A: Into<Value<T>>,
    B: Into<Value<T>>,
    C: Into<Value<T>>,
{
    map_2(a, b, c, |a, b| *a / *b, |a, c| *a / *c, |b, c| *b * *c)
}

#[cfg(test)]
mod tests {
    use super::div;
    use crate::{goal_vec, goals::unify, LVar};

    #[test]
    fn succeeds() {
        let (x, y, z) = (LVar::new(), LVar::new(), LVar::new());
        let goals = goal_vec![unify(x, 6), unify(y, 3), unify(z, 2), div(x, y, z)];
        goals.assert_permutations_resolve_to(&(x, y, z), vec![(6, 3, 2)]);
    }

    #[test]
    fn fails() {
        let (x, y, z) = (LVar::new(), LVar::new(), LVar::new());
        let goals = goal_vec![unify(x, 6), unify(y, 3), unify(z, 5), div(x, y, z)];
        goals.assert_permutations_resolve_to(&(x, y, z), vec![]);
    }
}
