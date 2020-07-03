use crate::goal::{unify, Goal};
use crate::lvec::LVec;
use crate::state::{
    constraints::{resolve_2, Constraint, ResolveFn, VarWatch},
    State,
};
use crate::value::{val, IntoVal, Val};
use crate::{DomainType, UnifyIn};
use std::fmt::Debug;
use std::iter::repeat;

/// Create a [`Goal`](canrun::Goal) that attempts to unify a `Val<T>` with
/// any of the items in a `LVec<T>`.
///
/// This goal will fork the state for each match found.
/// # Examples:
/// ```
/// use canrun::{Goal, val, var, all, unify};
/// use canrun::{lvec, example::Collections};
///
/// let needle = var();
/// let haystack = var();
/// let goal: Goal<Collections> = all![
///     unify(needle, lvec![1]),
///     unify(haystack, lvec![1, 2, 3]),
///     lvec::subset(needle, haystack),
/// ];
/// let results: Vec<_> = goal.query(needle).collect();
/// assert_eq!(results, vec![vec![1]]);
/// ```
pub fn subset<'a, I, SV, CV, D>(subset: SV, collection: CV) -> Goal<'a, D>
where
    I: UnifyIn<'a, D> + 'a,
    SV: IntoVal<LVec<I>>,
    LVec<I>: UnifyIn<'a, D>,
    CV: IntoVal<LVec<I>>,
    D: DomainType<'a, I> + DomainType<'a, LVec<I>>,
{
    Goal::constraint(Subset {
        subset: subset.into_val(),
        collection: collection.into_val(),
    })
}

#[derive(Debug)]
struct Subset<I: Debug> {
    subset: Val<LVec<I>>,
    collection: Val<LVec<I>>,
}

impl<'a, I, D> Constraint<'a, D> for Subset<I>
where
    I: UnifyIn<'a, D>,
    D: DomainType<'a, I> + DomainType<'a, LVec<I>>,
{
    fn attempt(&self, state: &State<'a, D>) -> Result<ResolveFn<'a, D>, VarWatch> {
        let (subset, collection) = resolve_2(&self.subset, &self.collection, state)?;

        let col_size = collection.len();
        let sub_size = subset.len();

        if col_size < sub_size {
            Ok(Box::new(|state| Goal::fail().apply(state)))
        } else {
            let subset: LVec<I> = subset.vec.iter().into();
            let goals: Vec<_> = (0..=col_size - sub_size)
                .zip(repeat(val!(subset)))
                .map(move |(index, subset)| {
                    // TODO: Add some sort of slicing concept to LVec that avoids creating new
                    // vectors
                    let superset: LVec<I> = collection.vec[index..index + sub_size].into();
                    unify(subset, superset) as Goal<D>
                })
                .collect();
            Ok(Box::new(|state| Goal::any(goals).apply(state)))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::example::Collections;
    use crate::goal::{either, unify, Goal};
    use crate::lvec;
    use crate::util;
    use crate::value::var;

    #[test]
    fn basic_subset() {
        let x = var::<i32>();
        let goals: Vec<Goal<Collections>> = vec![lvec::subset(lvec![x, 2], lvec![1, 2, 3])];
        util::assert_permutations_resolve_to(goals, x, vec![1]);
    }

    #[test]
    fn subset_with_conditions() {
        let x = var();
        let goals: Vec<Goal<Collections>> =
            vec![unify(x, 3), lvec::subset(lvec![2, x], lvec![1, 2, 3])];
        util::assert_permutations_resolve_to(goals, x, vec![3]);
    }

    #[test]
    fn unify_two_subsets_1() {
        let x = var();
        let list = lvec![1, 2, 3];
        let goals: Vec<Goal<Collections>> = vec![
            lvec::subset(lvec![1], x),
            lvec::subset(lvec![2], x),
            unify(x, list),
        ];
        util::assert_permutations_resolve_to(goals, x, vec![vec![1, 2, 3]]);
    }

    #[test]
    fn unify_two_subsets_2() {
        let x = var();
        let list = lvec![1, 2, 3];
        let goals: Vec<Goal<Collections>> = vec![
            lvec::subset(lvec![1], x),
            lvec::subset(lvec![2], x),
            unify(x, list),
        ];
        util::assert_permutations_resolve_to(goals, x, vec![vec![1, 2, 3]]);
    }

    #[test]
    fn unify_two_subsets_3() {
        let x = var();
        let list = lvec![1, 2, 3];
        let goals: Vec<Goal<Collections>> = vec![
            either(lvec::subset(lvec![1, 2], x), lvec::subset(lvec![4], x)),
            lvec::subset(lvec![2, 3], x),
            unify(x, list),
        ];
        util::assert_permutations_resolve_to(goals, x, vec![vec![1, 2, 3]]);
    }

    #[test]
    fn unify_two_subsets_4() {
        let x = var();
        let list = lvec![1, 2, 3];
        let goals: Vec<Goal<Collections>> = vec![
            lvec::subset(lvec![1, 2], x),
            lvec::subset(lvec![4], x),
            unify(x, list),
        ];

        util::assert_permutations_resolve_to(goals, x, vec![]);
    }
}
