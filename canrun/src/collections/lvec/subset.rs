use crate::constraints::{resolve_2, Constraint, ResolveFn};
use crate::goals::{unify, Any, Goal};
use crate::lvec::LVec;
use crate::{LVarList, State, Unify, Value};
use std::fmt::Debug;
use std::iter::repeat;
use std::rc::Rc;

/**
Assert that [`LVec`] `a` is a subset of [`LVec`] `b`.

This means that all of the items in `a` unify with a contiguous run of items in `b`.

This goal will fork the state for each match found.
# Examples:
```
use canrun::{LVar, all, unify, lvec, Query};

let needle = LVar::new();
let haystack = LVar::new();
let goal = all![
    unify(&needle, lvec![1]),
    unify(&haystack, lvec![1, 2, 3]),
    lvec::subset(&needle, haystack),
];
let results: Vec<_> = goal.query(needle).collect();
assert_eq!(results, vec![vec![1]]);
```
*/
pub fn subset<T, SV, CV>(subset: SV, collection: CV) -> Subset<T>
where
    T: Unify,
    SV: Into<Value<LVec<T>>>,
    LVec<T>: Unify,
    CV: Into<Value<LVec<T>>>,
{
    Subset {
        subset: subset.into(),
        collection: collection.into(),
    }
}
/**
Assert that [`LVec`] `a` is a subset of [`LVec`] `b`. Create with [`subset`].
*/
#[derive(Debug)]
pub struct Subset<T: Unify> {
    subset: Value<LVec<T>>,
    collection: Value<LVec<T>>,
}

impl<T: Unify> Clone for Subset<T> {
    fn clone(&self) -> Self {
        Self {
            subset: self.subset.clone(),
            collection: self.collection.clone(),
        }
    }
}

impl<T: Unify> Goal for Subset<T> {
    fn apply(&self, state: State) -> Option<State> {
        state.constrain(Rc::new(self.clone()))
    }
}

impl<T: Unify> Constraint for Subset<T> {
    fn attempt(&self, state: &State) -> Result<ResolveFn, LVarList> {
        let (subset, collection) = resolve_2(&self.subset, &self.collection, state)?;

        let col_size = collection.len();
        let sub_size = subset.len();

        if col_size < sub_size {
            Ok(Box::new(|_| None))
        } else {
            let subset: LVec<T> = subset.vec.clone().into();
            let goals: Vec<_> = (0..=col_size - sub_size)
                .zip(repeat(Value::new(subset)))
                .map(move |(index, subset)| {
                    // TODO: Add some sort of slicing concept to LVec that avoids creating new vectors
                    let superset: LVec<T> = collection.vec[index..index + sub_size].into();
                    Rc::new(unify(subset, superset)) as Rc<dyn Goal>
                })
                .collect();
            Ok(Box::new(|state| Any::from(goals).apply(state)))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::goal_vec;
    use crate::goals::{either, unify};
    use crate::{lvec, LVar};

    #[test]
    fn basic_subset() {
        let x = LVar::new();
        let goals = goal_vec![lvec::subset(lvec![x, 2], lvec![1, 2, 3])];
        goals.assert_permutations_resolve_to(&x, vec![1]);
    }

    #[test]
    fn subset_with_conditions() {
        let x = LVar::new();
        let goals = goal_vec![unify(x, 3), lvec::subset(lvec![2, x], lvec![1, 2, 3])];
        goals.assert_permutations_resolve_to(&x, vec![3]);
    }

    #[test]
    fn unify_two_subsets_1() {
        let x = LVar::new();
        let list = lvec![1, 2, 3];
        let goals = goal_vec![
            lvec::subset(lvec![1], x),
            lvec::subset(lvec![2], x),
            unify(x, list),
        ];
        goals.assert_permutations_resolve_to(&x, vec![vec![1, 2, 3]]);
    }

    #[test]
    fn unify_two_subsets_2() {
        let x = LVar::new();
        let list = lvec![1, 2, 3];
        let goals = goal_vec![
            lvec::subset(lvec![1], x),
            lvec::subset(lvec![2], x),
            unify(x, list),
        ];
        goals.assert_permutations_resolve_to(&x, vec![vec![1, 2, 3]]);
    }

    #[test]
    fn unify_two_subsets_3() {
        let x = LVar::new();
        let list = lvec![1, 2, 3];
        let goals = goal_vec![
            either(lvec::subset(lvec![1, 2], x), lvec::subset(lvec![4], x)),
            lvec::subset(lvec![2, 3], x),
            unify(x, list),
        ];
        goals.assert_permutations_resolve_to(&x, vec![vec![1, 2, 3]]);
    }

    #[test]
    fn unify_two_subsets_4() {
        let x = LVar::new();
        let list = lvec![1, 2, 3];
        let goals = goal_vec![
            lvec::subset(lvec![1, 2], x),
            lvec::subset(lvec![4], x),
            unify(x, list),
        ];

        goals.assert_permutations_resolve_to(&x, vec![]);
    }

    #[test]
    fn subset_against_smaller() {
        let x = LVar::new();
        let goals = goal_vec![lvec::subset(lvec![x, 2], lvec![1])];
        goals.assert_permutations_resolve_to(&x, vec![]);
    }

    #[test]
    fn debug_impl() {
        let goal = lvec::subset(lvec![1], lvec![1, 2]);
        assert_ne!(format!("{goal:?}"), "");
    }
}
