use canrun::goal::{project::Project, unify, Goal};
use canrun::state::{Constraint, State};
use canrun::value::{IntoVal, Val};
use canrun::{DomainType, UnifyIn};
use std::fmt;
use std::iter::repeat;

/// Create a [`Goal`](canrun::goal) that attempts to unify a `Val<T>` with any of the items in a `Vec<Val<T>>`.
///
/// This goal will fork the state for each match found.
///
/// # Examples:
/// ```
/// use canrun::{Goal, val, var, all, unify};
/// use canrun::domains::example::VecI32;
/// use canrun_collections::{lvec, member};
///
/// let x = var();
/// let xs = var();
/// let goal: Goal<VecI32> = all![
///     unify(x, 1),
///     unify(xs, lvec![1, 2, 3]),
///     member(x, xs),
/// ];
/// let results: Vec<_> = goal.query(x).collect();
/// assert_eq!(results, vec![1]);
/// ```
///
/// ```
/// # use canrun::{Goal, val, var, all, unify};
/// # use canrun::domains::example::VecI32;
/// # use canrun_collections::{lvec, member};
/// #
/// let x = var();
/// let goal: Goal<VecI32> = all![
///     member(x, lvec![1, 2, 3]),
/// ];
/// let results: Vec<_> = goal.query(x).collect();
/// assert_eq!(results, vec![1, 2, 3]);
/// ```
pub fn member<'a, I, IV, CV, D>(item: IV, collection: CV) -> Goal<'a, D>
where
    I: UnifyIn<'a, D> + 'a,
    IV: IntoVal<I>,
    Vec<Val<I>>: UnifyIn<'a, D>,
    CV: IntoVal<Vec<Val<I>>>,
    D: DomainType<'a, I> + DomainType<'a, Vec<Val<I>>>,
{
    Goal::project(Member {
        item: item.into_val(),
        collection: collection.into_val(),
    })
}

struct Member<I> {
    item: Val<I>,
    collection: Val<Vec<Val<I>>>,
}

impl<'a, I, D> Project<'a, D> for Member<I>
where
    I: UnifyIn<'a, D>,
    D: DomainType<'a, I> + DomainType<'a, Vec<Val<I>>>,
{
    fn attempt<'r>(&'r self, state: State<'a, D>) -> Constraint<State<'a, D>> {
        let collection = state.resolve_val(&self.collection).resolved();
        match collection {
            Ok(collection) => {
                let goals: Vec<_> = collection
                    .iter()
                    .zip(repeat(self.item.clone()))
                    .map(|(a, b)| unify::<I, &Val<I>, Val<I>, D>(a, b) as Goal<D>)
                    .collect();
                Constraint::done(Goal::any(goals).apply(state))
            }
            Err(var) => Constraint::on_1(state, var),
        }
    }
}

impl<I> fmt::Debug for Member<I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Member ??")
    }
}

#[cfg(test)]
mod tests {
    use super::member;
    use crate::lvec;
    use canrun::domains::example::VecI32;
    use canrun::goal::{either, unify, Goal};
    use canrun::util;
    use canrun::value::var;

    #[test]
    fn basic_member() {
        let x = var::<i32>();
        let goals: Vec<Goal<VecI32>> = vec![member(x, lvec![1, 2, 3])];
        util::assert_permutations_resolve_to(goals, x, vec![1, 2, 3]);
    }

    #[test]
    fn member_with_conditions() {
        let x = var();
        let goals: Vec<Goal<VecI32>> = vec![unify(x, 2), member(x, lvec![1, 2, 3])];
        util::assert_permutations_resolve_to(goals, x, vec![2]);
    }

    #[test]
    fn unify_two_contains_1() {
        let x = var();
        let list = lvec![1, 2, 3];
        let goals: Vec<Goal<VecI32>> = vec![member(1, x), member(1, x), unify(x, list.clone())];
        util::assert_permutations_resolve_to(goals, x, vec![vec![1, 2, 3]]);
    }

    #[test]
    fn unify_two_contains_2() {
        let x = var();
        let list = lvec![1, 2, 3];
        let goals: Vec<Goal<VecI32>> = vec![member(1, x), member(2, x), unify(x, list.clone())];
        util::assert_permutations_resolve_to(goals, x, vec![vec![1, 2, 3]]);
    }

    #[test]
    fn unify_two_contains_3() {
        let x = var();
        let list = lvec![1, 2, 3];
        let goals: Vec<Goal<VecI32>> = vec![
            either(member(1, x), member(4, x)),
            member(2, x),
            unify(x, list.clone()),
        ];
        util::assert_permutations_resolve_to(goals, x, vec![vec![1, 2, 3]]);
    }

    #[test]
    fn unify_two_contains_4() {
        let x = var();
        let list = lvec![1, 2, 3];
        let goals: Vec<Goal<VecI32>> = vec![member(1, x), member(4, x), unify(x, list.clone())];

        util::assert_permutations_resolve_to(goals, x, vec![]);
    }
}
