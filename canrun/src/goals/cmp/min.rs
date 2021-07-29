use crate::cmp::{lt, lte};
use crate::goals::Goal;
use crate::value::IntoVal;
use crate::{both, either, unify, val};
use crate::{DomainType, UnifyIn};

/** Get the lesser of two values according to [`std::cmp::min`].

# Example:
```
use canrun::{unify, util, var, all, Goal};
use canrun::example::I32;
use canrun::cmp::min;

let (x, y, z) = (var(), var(), var());
let goal: Goal<I32> = all![
    unify(x, 1),
    unify(y, 2),
    unify(z, 1),
    min(x, y, z),
];
let results: Vec<_> = goal.query((x, y, z)).collect();
assert_eq!(results, vec![(1, 2, 1)]);
```
*/
pub fn min<'a, T, A, B, C, D>(a: A, b: B, c: C) -> Goal<'a, D>
where
    T: PartialOrd + UnifyIn<'a, D> + 'a,
    A: IntoVal<T>,
    B: IntoVal<T>,
    C: IntoVal<T>,
    D: DomainType<'a, T>,
{
    let a = val!(a);
    let b = val!(b);
    let c = val!(c);
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
    use crate::example::I32;
    use crate::{unify, util, var, Goal};

    #[test]
    fn succeeds_lt() {
        let (x, y, z) = (var(), var(), var());
        let goals: Vec<Goal<I32>> = vec![unify(x, 1), unify(y, 2), unify(z, 1), min(x, y, z)];
        util::assert_permutations_resolve_to(goals, (x, y, z), vec![(1, 2, 1)]);
    }
    #[test]
    fn succeeds_lte() {
        let (x, y, z) = (var(), var(), var());
        let goals: Vec<Goal<I32>> = vec![unify(x, 1), unify(y, 1), unify(z, 1), min(x, y, z)];
        util::assert_permutations_resolve_to(goals, (x, y, z), vec![(1, 1, 1)]);
    }

    #[test]
    fn fails() {
        let (x, y, z) = (var(), var(), var());
        let goals: Vec<Goal<I32>> = vec![unify(x, 2), unify(y, 1), unify(z, 2), min(x, y, z)];
        util::assert_permutations_resolve_to(goals, (x, y), vec![]);
    }
}
