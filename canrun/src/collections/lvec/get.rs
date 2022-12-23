use crate::goals::{unify, Goal};
use crate::lvec::LVec;
use crate::state::{
    constraints::{resolve_2, Constraint, ResolveFn, VarWatch},
    State,
};
use crate::value::{IntoVal, Val};
use crate::{DomainType, UnifyIn};
use std::fmt::Debug;

/// Create a [`Goal`] that attempts to unify a `Val<T>` with
/// the item at a specific index in a `LVec<T>`.

/// # Examples:
/// ```
/// use canrun::{Goal, val, LVar, var, all, unify, lvec, example::Collections};
///
/// let needle = var();
/// let index = var();
/// let haystack = var();
/// let goal: Goal<Collections> = all![
///     unify(index, 0),
///     unify(haystack, lvec![1, 2, 3]),
///     lvec::get(needle, index, haystack),
/// ];
/// let results: Vec<_> = goal.query(needle).collect();
/// assert_eq!(results, vec![1]);
/// ```
pub fn get<'a, I, IV, IdxV, CV, D>(item: IV, index: IdxV, collection: CV) -> Goal<'a, D>
where
    I: UnifyIn<'a, D> + 'a,
    LVec<I>: UnifyIn<'a, D>,
    IV: IntoVal<I>,
    IdxV: IntoVal<usize>,
    CV: IntoVal<LVec<I>>,
    D: DomainType<'a, usize> + DomainType<'a, I> + DomainType<'a, LVec<I>>,
{
    Goal::constraint(Get {
        item: item.into_val(),
        index: index.into_val(),
        collection: collection.into_val(),
    })
}

#[derive(Debug)]
struct Get<I: Debug> {
    item: Val<I>,
    index: Val<usize>,
    collection: Val<LVec<I>>,
}

impl<'a, I, D> Constraint<'a, D> for Get<I>
where
    I: UnifyIn<'a, D> + 'a,
    D: DomainType<'a, usize> + DomainType<'a, I> + DomainType<'a, LVec<I>>,
{
    fn attempt(&self, state: &State<'a, D>) -> Result<ResolveFn<'a, D>, VarWatch> {
        let (index, collection) = resolve_2(&self.index, &self.collection, state)?;
        let item: Val<I> = self.item.clone();
        let found = collection.vec.get(*index);

        match found {
            Some(found) => {
                let found: Val<I> = found.clone();
                Ok(Box::new(move |state| {
                    unify::<I, Val<I>, Val<I>, D>(item, found).apply(state)
                }))
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
    fn basic_get() {
        let x = var::<i32>();
        let goals: Vec<Goal<Collections>> = vec![lvec::get(x, 0, lvec![1, 2, 3])];
        util::assert_permutations_resolve_to(goals, x, vec![1]);
    }

    #[test]
    fn get_with_lh_var() {
        let x = var();
        let goals: Vec<Goal<Collections>> = vec![unify(x, 2), lvec::get(x, 1, lvec![1, 2, 3])];
        util::assert_permutations_resolve_to(goals, x, vec![2]);
    }

    #[test]
    fn get_with_rh_var() {
        let x = var();
        let goals: Vec<Goal<Collections>> = vec![unify(x, 3), lvec::get(3, 2, lvec![1, 2, x])];
        util::assert_permutations_resolve_to(goals, x, vec![3]);
    }

    #[test]
    fn get_fails() {
        let x = var::<i32>();
        let goals: Vec<Goal<Collections>> = vec![lvec::get(x, 3, lvec![1, 2])];
        util::assert_permutations_resolve_to(goals, x, vec![]);
    }

    #[test]
    fn debug_impl() {
        let x = var::<i32>();
        let goal: Goal<Collections> = lvec::get(x, 0, lvec![1, 2, 3]);
        assert_ne!(format!("{goal:?}"), "");
    }
}
