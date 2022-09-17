use crate::goals::{unify, Goal};
use crate::lvec::LVec;
use crate::state::{
    constraints::{resolve_1, Constraint, ResolveFn, VarWatch},
    State,
};
use crate::value::{IntoVal, Val};
use crate::{DomainType, UnifyIn};
use std::fmt::Debug;
use std::iter::repeat;

/** Create a [`Goal`] that attempts to unify a `Val<T>` with
any of the items in a `LVec<T>`.

This goal will fork the state for each match found.

# Examples:
```
use canrun::{Goal, val, var, all, unify, lvec, example::Collections};

let x = var();
let xs = var();
let goal: Goal<Collections> = all![
    unify(x, 1),
    unify(xs, lvec![1, 2, 3]),
    lvec::member(x, xs),
];
let results: Vec<_> = goal.query(x).collect();
assert_eq!(results, vec![1]);
```

```
# use canrun::{Goal, val, var, all, unify};
use canrun::{lvec, example::Collections};
#
let x = var();
let goal: Goal<Collections> = all![
    lvec::member(x, lvec![1, 2, 3]),
];
let results: Vec<_> = goal.query(x).collect();
assert_eq!(results, vec![1, 2, 3]);
```
*/
pub fn member<'a, I, IV, CV, D>(item: IV, collection: CV) -> Goal<'a, D>
where
    I: UnifyIn<'a, D> + 'a,
    IV: IntoVal<I>,
    LVec<I>: UnifyIn<'a, D>,
    CV: IntoVal<LVec<I>>,
    D: DomainType<'a, I> + DomainType<'a, LVec<I>>,
{
    Goal::constraint(Member {
        item: item.into_val(),
        collection: collection.into_val(),
    })
}

#[derive(Debug)]
struct Member<I: Debug> {
    item: Val<I>,
    collection: Val<LVec<I>>,
}

impl<'a, I, D> Constraint<'a, D> for Member<I>
where
    I: UnifyIn<'a, D>,
    D: DomainType<'a, I> + DomainType<'a, LVec<I>>,
{
    fn attempt(&self, state: &State<'a, D>) -> Result<ResolveFn<'a, D>, VarWatch> {
        let collection = resolve_1(&self.collection, state)?;
        let goals: Vec<_> = collection
            .vec
            .iter()
            .zip(repeat(self.item.clone()))
            .map(|(a, b)| unify::<I, &Val<I>, Val<I>, D>(a, b) as Goal<D>)
            .collect();
        Ok(Box::new(|state| Goal::any(goals).apply(state)))
    }
}

#[cfg(test)]
mod tests {
    use crate::example::Collections;
    use crate::goals::{either, unify, Goal};
    use crate::lvec;
    use crate::util;
    use crate::value::var;

    #[test]
    fn basic_member() {
        let x = var::<i32>();
        let goals: Vec<Goal<Collections>> = vec![lvec::member(x, lvec![1, 2, 3])];
        util::assert_permutations_resolve_to(goals, x, vec![1, 2, 3]);
    }

    #[test]
    fn member_with_conditions() {
        let x = var();
        let goals: Vec<Goal<Collections>> = vec![unify(x, 2), lvec::member(x, lvec![1, 2, 3])];
        util::assert_permutations_resolve_to(goals, x, vec![2]);
    }

    #[test]
    fn unify_two_contains_1() {
        let x = var();
        let list = lvec![1, 2, 3];
        let goals: Vec<Goal<Collections>> =
            vec![lvec::member(1, x), lvec::member(1, x), unify(x, list)];
        util::assert_permutations_resolve_to(goals, x, vec![vec![1, 2, 3]]);
    }

    #[test]
    fn unify_two_contains_2() {
        let x = var();
        let list = lvec![1, 2, 3];
        let goals: Vec<Goal<Collections>> =
            vec![lvec::member(1, x), lvec::member(2, x), unify(x, list)];
        util::assert_permutations_resolve_to(goals, x, vec![vec![1, 2, 3]]);
    }

    #[test]
    fn unify_two_contains_3() {
        let x = var();
        let list = lvec![1, 2, 3];
        let goals: Vec<Goal<Collections>> = vec![
            either(lvec::member(1, x), lvec::member(4, x)),
            lvec::member(2, x),
            unify(x, list),
        ];
        util::assert_permutations_resolve_to(goals, x, vec![vec![1, 2, 3]]);
    }

    #[test]
    fn unify_two_contains_4() {
        let x = var();
        let list = lvec![1, 2, 3];
        let goals: Vec<Goal<Collections>> =
            vec![lvec::member(1, x), lvec::member(4, x), unify(x, list)];

        util::assert_permutations_resolve_to(goals, x, vec![]);
    }

    #[test]
    fn debug_impl() {
        let goal: Goal<Collections> = lvec::member(1, lvec![1, 2]);
        assert_ne!(format!("{:?}", goal), "")
    }
}
