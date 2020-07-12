use crate::goals::{unify, Goal};
use crate::lvec::LVec;
use crate::state::{
    constraints::{resolve_2, Constraint, ResolveFn, VarWatch},
    State,
};
use crate::value::{IntoVal, Val};
use crate::{DomainType, UnifyIn};
use std::fmt::Debug;
use std::ops::Range;

/// Create a [`Goal`] that attempts to unify an `LVec<T>` with
/// a slice from another `LVec<T>` defined by a ['Range'].
///
/// # Examples:
/// ```
/// use canrun::{Goal, val, var, all, unify, lvec, example::Collections};
///
/// let needle = var();
/// let haystack = var();
/// let range = var();
/// let goal: Goal<Collections> = all![
///     unify(range, 0..2),
///     unify(haystack, lvec![1, 2, 3]),
///     lvec::slice(needle, range, haystack),
/// ];
/// let results: Vec<_> = goal.query(needle).collect();
/// assert_eq!(results, vec![vec![1, 2]]);
/// ```
pub fn slice<'a, I, SV, RV, CV, D>(slice: SV, range: RV, collection: CV) -> Goal<'a, D>
where
    I: UnifyIn<'a, D> + 'a,
    LVec<I>: UnifyIn<'a, D>,
    SV: IntoVal<LVec<I>>,
    RV: IntoVal<Range<usize>>,
    CV: IntoVal<LVec<I>>,
    D: DomainType<'a, Range<usize>> + DomainType<'a, I> + DomainType<'a, LVec<I>>,
{
    Goal::constraint(Slice {
        slice: slice.into_val(),
        range: range.into_val(),
        collection: collection.into_val(),
    })
}

#[derive(Debug)]
struct Slice<I: Debug> {
    slice: Val<LVec<I>>,
    range: Val<Range<usize>>,
    collection: Val<LVec<I>>,
}

impl<'a, I, D> Constraint<'a, D> for Slice<I>
where
    I: UnifyIn<'a, D> + 'a,
    D: DomainType<'a, Range<usize>> + DomainType<'a, I> + DomainType<'a, LVec<I>>,
{
    fn attempt(&self, state: &State<'a, D>) -> Result<ResolveFn<'a, D>, VarWatch> {
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
    use crate::example::Collections;
    use crate::goals::{unify, Goal};
    use crate::lvec;
    use crate::util;
    use crate::value::var;

    #[test]
    fn basic_slice() {
        let x = var::<i32>();
        let goals: Vec<Goal<Collections>> = vec![lvec::slice(lvec![x], 0..1, lvec![1, 2, 3])];
        util::assert_permutations_resolve_to(goals, x, vec![1]);
    }

    #[test]
    fn slice_with_lh_var() {
        let x = var();
        let goals: Vec<Goal<Collections>> =
            vec![unify(x, 3), lvec::slice(lvec![2, x], 1..3, lvec![1, 2, 3])];
        util::assert_permutations_resolve_to(goals, x, vec![3]);
    }

    #[test]
    fn slice_with_rh_var() {
        let x = var();
        let goals: Vec<Goal<Collections>> =
            vec![unify(x, 3), lvec::slice(lvec![2, 3], 1..3, lvec![1, 2, x])];
        util::assert_permutations_resolve_to(goals, x, vec![3]);
    }

    #[test]
    fn slice_with_out_of_range() {
        let x = var();
        let goals: Vec<Goal<Collections>> =
            vec![unify(x, 3), lvec::slice(lvec![2, 3], 1..4, lvec![1, 2, x])];
        // Is simply failing the goal what we want here?
        util::assert_permutations_resolve_to(goals, x, vec![]);
    }
}
