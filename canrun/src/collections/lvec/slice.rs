use crate::constraints::{resolve_2, Constraint, ResolveFn};
use crate::goals::{unify, Goal};
use crate::lvec::LVec;
use crate::{LVarList, State, Unify, Value};
use std::fmt::Debug;
use std::ops::Range;
use std::rc::Rc;

/// Create a [`Goal`] that attempts to unify an `LVec<T>` with
/// a slice from another `LVec<T>` defined by a [`Range`].
///
/// # Examples:
/// ```
/// use canrun::{all, unify, lvec, LVar, Query};
///
/// let needle = LVar::new();
/// let haystack = LVar::new();
/// let range = LVar::new();
/// let goal = all![
///     unify(&range, 0..2),
///     unify(&haystack, lvec![1, 2, 3]),
///     lvec::slice(&needle, &range, &haystack),
/// ];
/// let results: Vec<_> = goal.query(needle).collect();
/// assert_eq!(results, vec![vec![1, 2]]);
/// ```
pub fn slice<T>(
    slice: impl Into<Value<LVec<T>>>,
    range: impl Into<Value<Range<usize>>>,
    collection: impl Into<Value<LVec<T>>>,
) -> Slice<T>
where
    T: Unify,
    LVec<T>: Unify,
{
    Slice {
        slice: slice.into(),
        range: range.into(),
        collection: collection.into(),
    }
}

/// A [`Goal`] that attempts to unify an `LVec<T>` with
/// a slice from another `LVec<T>` defined by a [`Range`]. Create with [`slice()`].
#[derive(Debug)]
pub struct Slice<T: Unify> {
    slice: Value<LVec<T>>,
    range: Value<Range<usize>>,
    collection: Value<LVec<T>>,
}

impl<T: Unify> Clone for Slice<T> {
    fn clone(&self) -> Self {
        Self {
            slice: self.slice.clone(),
            range: self.range.clone(),
            collection: self.collection.clone(),
        }
    }
}

impl<T: Unify> Goal for Slice<T> {
    fn apply(&self, state: State) -> Option<State> {
        state.constrain(Rc::new(self.clone()))
    }
}

impl<T: Unify> Constraint for Slice<T>
where
    T: Unify,
{
    fn attempt(&self, state: &State) -> Result<ResolveFn, LVarList> {
        let (range, collection) = resolve_2(&self.range, &self.collection, state)?;
        let slice_a = self.slice.clone();
        let slice_b = collection.vec.get((*range).clone());
        match slice_b {
            Some(slice_b) => {
                let slice_b: LVec<_> = slice_b.into();
                Ok(Box::new(move |state| unify(slice_a, slice_b).apply(state)))
            }
            None => Ok(Box::new(|_| None)), // need a better way to fail
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::goal_vec;
    use crate::goals::unify;
    use crate::{lvec, LVar};

    #[test]
    fn basic_slice() {
        let x = LVar::new();
        let goals = goal_vec![lvec::slice(lvec![x], 0..1, lvec![1, 2, 3])];
        goals.assert_permutations_resolve_to(&x, vec![1]);
    }

    #[test]
    fn slice_with_lh_var() {
        let x = LVar::new();
        let goals = goal_vec![unify(x, 3), lvec::slice(lvec![2, x], 1..3, lvec![1, 2, 3])];
        goals.assert_permutations_resolve_to(&x, vec![3]);
    }

    #[test]
    fn slice_with_rh_var() {
        let x = LVar::new();
        let goals = goal_vec![unify(x, 3), lvec::slice(lvec![2, 3], 1..3, lvec![1, 2, x])];
        goals.assert_permutations_resolve_to(&x, vec![3]);
    }

    #[test]
    fn slice_with_out_of_range() {
        let x = LVar::new();
        let goals = goal_vec![unify(x, 3), lvec::slice(lvec![2, 3], 1..4, lvec![1, 2, x])];
        // Is simply failing the goal what we want here?
        goals.assert_permutations_resolve_to(&x, vec![]);
    }

    #[test]
    fn debug_impl() {
        let slice = lvec::slice(lvec![2, 3], 1..4, lvec![1, 2, 3]);
        assert_ne!(format!("{slice:?}"), "");
    }
}
