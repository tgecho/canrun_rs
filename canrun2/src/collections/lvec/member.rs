use crate::core::{
    constraints::{resolve_1, Constraint, ResolveFn, VarWatch},
    State, Unify, Value,
};
use crate::goals::unify;
use crate::goals::Any;
use crate::goals::Goal;
use std::fmt::Debug;
use std::iter::repeat;
use std::rc::Rc;

use super::LVec;

/** Create a [`Goal`] that attempts to unify a `Value<T>` with
any of the items in a `LVec<T>`.

This goal will fork the state for each match found.

# Examples:
```
use canrun2::{LVar, all, unify, lvec, Query};

let x = LVar::new();
let xs = LVar::new();
let goal = all![
    unify(&x, 1),
    unify(&xs, lvec![1, 2, 3]),
    lvec::member(x, xs),
];
let results: Vec<_> = goal.query(x).collect();
assert_eq!(results, vec![1]);
```

```
# use canrun2::{LVar, all, unify, lvec, Query};
let x = LVar::new();
let goal = all![
    lvec::member(x, lvec![1, 2, 3]),
];
let results: Vec<_> = goal.query(x).collect();
assert_eq!(results, vec![1, 2, 3]);
```
*/
pub fn member<T, IntoT, IntoLVecT>(item: IntoT, collection: IntoLVecT) -> Member<T>
where
    T: Unify,
    IntoT: Into<Value<T>>,
    IntoLVecT: Into<Value<LVec<T>>>,
{
    Member {
        item: item.into(),
        collection: collection.into(),
    }
}

/** A [`Goal`] that attempts to unify a `Value<T>` with
any of the items in a `LVec<T>`. Create with [`member`].
*/
#[derive(Debug)]
pub struct Member<T: Unify> {
    item: Value<T>,
    collection: Value<LVec<T>>,
}

impl<T: Unify> Goal for Member<T> {
    fn apply(&self, state: State) -> Option<State> {
        state.constrain(Rc::new(self.clone()))
    }
}

impl<T: Unify> Clone for Member<T> {
    fn clone(&self) -> Self {
        Self {
            item: self.item.clone(),
            collection: self.collection.clone(),
        }
    }
}

impl<T: Unify> Constraint for Member<T> {
    fn attempt(&self, state: &State) -> Result<ResolveFn, VarWatch> {
        let collection = resolve_1(&self.collection, state)?;
        let any = collection
            .vec
            .iter()
            .zip(repeat(self.item.clone()))
            .map(|(a, b)| Rc::new(unify(a, b)) as Rc<dyn Goal>)
            .collect::<Any>();
        Ok(Box::new(move |state| any.apply(state)))
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::{
        core::LVar,
        core::Query,
        goal_vec,
        goals::{either, unify},
        lvec,
    };

    use super::member;

    #[test]
    fn basic_member() {
        let x = LVar::new();
        let goal = member(x, lvec![1, 2, 3]);
        let results = goal.query(x).collect::<Vec<_>>();
        assert_eq!(results, vec![1, 2, 3]);
    }

    #[test]
    fn member_with_conditions() {
        let x = LVar::new();
        let goals = goal_vec![unify(x, 2), member(x, lvec![1, 2, 3])];
        goals.assert_permutations_resolve_to(x, vec![2]);
    }

    #[test]
    fn unify_two_contains_1() {
        let x = LVar::new();
        let list = lvec![1, 2, 3];
        let goals = goal_vec![member(1, &x), member(1, &x), unify(&x, list)];
        goals.assert_permutations_resolve_to(x, vec![vec![1, 2, 3]]);
    }

    #[test]
    fn unify_two_contains_2() {
        let x = LVar::new();
        let list = lvec![1, 2, 3];
        let goals = goal_vec![member(1, &x), member(2, &x), unify(&x, list)];
        goals.assert_permutations_resolve_to(x, vec![vec![1, 2, 3]]);
    }

    #[test]
    fn unify_two_contains_3() {
        let x = LVar::new();
        let list = lvec![1, 2, 3];
        let goals = goal_vec![
            either(member(1, &x), member(4, &x)),
            member(2, &x),
            unify(&x, list),
        ];
        goals.assert_permutations_resolve_to(x, vec![vec![1, 2, 3]]);
    }

    #[test]
    fn unify_two_contains_4() {
        let x = LVar::new();
        let list = lvec![1, 2, 3];
        let goals = goal_vec![member(1, &x), member(4, &x), unify(&x, list)];

        goals.assert_permutations_resolve_to(x, vec![]);
    }
}
