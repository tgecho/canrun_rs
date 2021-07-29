use crate::goals::Goal;
use crate::map_2;
use crate::value::IntoVal;
use crate::{DomainType, UnifyIn};
use std::ops::{Add, Sub};

/** Add two values together.

# Example:
```
use canrun::{unify, util, var, Goal};
use canrun::example::I32;
use canrun::ops::add;

let x = var();
let goal: Goal<I32> = add(1, 2, x);
let results: Vec<_> = goal.query(x).collect();
assert_eq!(results, vec![3]);
```
*/
pub fn add<'a, T, A, B, C, D>(a: A, b: B, c: C) -> Goal<'a, D>
where
    T: Add<Output = T> + Sub<Output = T> + UnifyIn<'a, D> + Copy + 'a,
    A: IntoVal<T>,
    B: IntoVal<T>,
    C: IntoVal<T>,
    D: DomainType<'a, T>,
{
    map_2(a, b, c, |a, b| *a + *b, |a, c| *c - *a, |b, c| *c - *b)
}

#[cfg(test)]
mod tests {
    use super::add;
    use crate::example::I32;
    use crate::{unify, util, var, Goal};

    #[test]
    fn succeeds() {
        let (x, y, z) = (var(), var(), var());
        let goals: Vec<Goal<I32>> = vec![unify(x, 1), unify(y, 2), unify(z, 3), add(x, y, z)];
        util::assert_permutations_resolve_to(goals, (x, y, z), vec![(1, 2, 3)]);
    }

    #[test]
    fn fails() {
        let (x, y, z) = (var(), var(), var());
        let goals: Vec<Goal<I32>> = vec![unify(x, 1), unify(y, 2), unify(z, 4), add(x, y, z)];
        util::assert_permutations_resolve_to(goals, (x, y, z), vec![]);
    }
}
