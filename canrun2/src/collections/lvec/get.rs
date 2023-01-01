use super::LVec;
use crate::{
    core::{
        constraints::{resolve_2, Constraint, ResolveFn, VarWatch},
        State, Unify, Value,
    },
    goals::Goal,
};
use std::{fmt::Debug, rc::Rc};

/**
Create a [`Goal`] that attempts to unify a `Value<T>` with
the item at a specific index in a `LVec<T>`. Create with [`get`]. */
#[derive(Debug)]
pub struct Get<T: Unify> {
    item: Value<T>,
    index: Value<usize>,
    collection: Value<LVec<T>>,
}

impl<T: Unify> Clone for Get<T> {
    fn clone(&self) -> Self {
        Self {
            item: self.item.clone(),
            index: self.index.clone(),
            collection: self.collection.clone(),
        }
    }
}

/**
Create a [`Goal`] that attempts to unify a `Value<T>` with
the item at a specific index in a `LVec<T>`.

# Examples:
```
use canrun2::{LVar, all, unify, lvec, Query};

let needle = LVar::new();
let index = LVar::new();
let haystack = LVar::new();
let goal = all![
    unify(index, 0),
    unify(&haystack, lvec![1, 2, 3]),
    lvec::get(needle, index, haystack),
];
let results: Vec<_> = goal.query(needle).collect();
assert_eq!(results, vec![1]);
```
*/
pub fn get<T, IntoT, Index, Collection>(item: IntoT, index: Index, collection: Collection) -> Get<T>
where
    T: Unify,
    IntoT: Into<Value<T>>,
    Index: Into<Value<usize>>,
    Collection: Into<Value<LVec<T>>>,
{
    Get {
        item: item.into(),
        index: index.into(),
        collection: collection.into(),
    }
}

impl<T: Unify> Goal for Get<T> {
    fn apply(&self, state: State) -> Option<State> {
        state.constrain(Rc::new(self.clone()))
    }
}

impl<T: Unify> Constraint for Get<T> {
    fn attempt(&self, state: &State) -> Result<ResolveFn, VarWatch> {
        let (index, collection) = resolve_2(&self.index, &self.collection, state)?;
        let item = self.item.clone();
        let found = collection.vec.get(*index);

        match found {
            Some(found) => {
                let found: Value<T> = found.clone();
                Ok(Box::new(move |state| state.unify(&item, &found)))
            }
            None => Ok(Box::new(|_| None)), // need a better way to fail
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{core::LVar, core::Query, lvec};

    #[test]
    fn basic_get() {
        let x = LVar::new();
        let goal = get(x, 0, lvec![1, 2, 3]);
        assert_eq!(goal.query(x).collect::<Vec<_>>(), vec![1]);
    }

    #[test]
    fn get_with_lh_var() {
        let x = LVar::new();
        let goal = get(x, 1, lvec![1, 2, 3]);
        assert_eq!(goal.query(x).collect::<Vec<_>>(), vec![2]);
    }

    #[test]
    fn get_with_rh_var() {
        let x = LVar::new();
        let goal = get(3, 2, lvec![1, 2, x]);
        assert_eq!(goal.query(x).collect::<Vec<_>>(), vec![3]);
    }

    #[test]
    fn get_fails() {
        let x = LVar::new();
        let goal = get(x, 3, lvec![1, 2]);
        assert_eq!(goal.query(x).collect::<Vec<_>>(), vec![]);
    }
}
