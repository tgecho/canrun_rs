use std::{collections::HashSet, fmt::Debug};

use itertools::Itertools;

use super::State;
use crate::{
    core::{LVar, VarId},
    resolve_any, AnyVal,
};

/**
An opaque list of untyped [`LVar`]s.

This is usually used to set up a watch on behalf of a [`Constraint`](crate::constraints::Constraint).
Consider generating this with the [`resolve_1`](crate::constraints::resolve_1), [`resolve_2`](crate::constraints::resolve_2), [`OneOfTwo`](crate::constraints::OneOfTwo)
or [`TwoOfThree`](crate::constraints::TwoOfThree) helpers.

It has to be opaque because each [`LVar`] might have a different type parameter, making it impossible to store them all in a `Vec<LVar<T>>`.

It is also the return value of [`State::vars()`].
*/
#[derive(Debug)]
pub struct LVarList(pub(crate) Vec<VarId>);

impl PartialEq for LVarList {
    fn eq(&self, other: &Self) -> bool {
        if self.0.len() != other.0.len() {
            return false;
        }
        let lookup: HashSet<_> = self.0.iter().collect();
        other.0.iter().all(|id| lookup.contains(id))
    }
}

impl LVarList {
    /// Create an `LVarList` from a single [`LVar`].
    pub fn one<A>(a: &LVar<A>) -> Self {
        LVarList(vec![a.id])
    }

    /// Create an `LVarList` from two [`LVar`]s.
    pub fn two<A, B>(a: &LVar<A>, b: &LVar<B>) -> Self {
        LVarList(vec![a.id, b.id])
    }

    /// Generate a new `LVarList` based on `&self` with any variables that have
    /// been been resolved in the passed in state removed.
    #[must_use]
    pub fn without_resolved_in(&self, state: &State) -> LVarList {
        LVarList(
            self.0
                .iter()
                .filter_map(|id| {
                    if resolve_any(&state.values, &AnyVal::Var(*id)).is_resolved() {
                        None
                    } else {
                        Some(*id)
                    }
                })
                .collect(),
        )
    }

    /// Returns the number of [`LVar`]s.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns true if the `LVarList` contains no elements.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Produce a single deduplicated `LVarList` from an [`Iterator`] of `LVarList`s.
    pub fn flatten(lists: impl Iterator<Item = LVarList>) -> LVarList {
        LVarList(lists.flat_map(|list| list.0.into_iter()).unique().collect())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lvarlist_len() {
        assert_eq!(LVarList(vec![]).len(), 0);
        assert_eq!(LVarList(vec![1]).len(), 1);
    }
}
