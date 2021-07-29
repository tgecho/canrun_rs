use crate::goals::Goal;
use crate::map_2;
use crate::value::IntoVal;
use crate::{DomainType, UnifyIn};
use std::ops::{Div, Mul};

/** Divide one value with another.

# Example:
```
use canrun::{unify, util, var, Goal};
use canrun::example::I32;
use canrun::ops::div;

let x = var();
let goal: Goal<I32> = div(3, 2, x);
let results: Vec<_> = goal.query(x).collect();
assert_eq!(results, vec![1]);
```
*/
pub fn div<'a, T, A, B, C, D>(a: A, b: B, c: C) -> Goal<'a, D>
where
    T: Mul<Output = T> + Div<Output = T> + UnifyIn<'a, D> + Copy + 'a,
    A: IntoVal<T>,
    B: IntoVal<T>,
    C: IntoVal<T>,
    D: DomainType<'a, T>,
{
    map_2(a, b, c, |a, b| *a / *b, |a, c| *a / *c, |b, c| *b * *c)
}

#[cfg(test)]
mod tests {
    use super::div;
    use crate::example::I32;
    use crate::{unify, util, var, Goal};

    #[test]
    fn succeeds() {
        let (x, y, z) = (var(), var(), var());
        let goals: Vec<Goal<I32>> = vec![unify(x, 6), unify(y, 3), unify(z, 2), div(x, y, z)];
        util::assert_permutations_resolve_to(goals, (x, y, z), vec![(6, 3, 2)]);
    }

    #[test]
    fn fails() {
        let (x, y, z) = (var(), var(), var());
        let goals: Vec<Goal<I32>> = vec![unify(x, 6), unify(y, 3), unify(z, 5), div(x, y, z)];
        util::assert_permutations_resolve_to(goals, (x, y, z), vec![]);
    }
}
