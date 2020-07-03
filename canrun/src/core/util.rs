//! Assorted helpers, especially for testing.
pub(super) mod multikeymultivaluemap;

use crate::Domain;
use crate::Goal;
use crate::Query;
use crate::ReifyIn;
use crate::State;
use itertools::Itertools;
use std::fmt::Debug;

pub(crate) fn all_permutations<'a, D>(
    goals: Vec<Goal<'a, D>>,
) -> impl Iterator<Item = Vec<Goal<'a, D>>> + 'a
where
    D: Domain<'a> + 'a,
{
    let goals_len = goals.len();
    goals.into_iter().permutations(goals_len)
}

pub(crate) fn goals_resolve_to<'a, D, Q>(goals: &[Goal<'a, D>], query: Q) -> Vec<Q::Reified>
where
    D: Domain<'a> + 'a,
    Q: ReifyIn<'a, D> + 'a,
{
    let goal = Goal::all(goals.to_owned());
    goal_resolves_to(goal, query)
}

pub(crate) fn goal_resolves_to<'a, D, Q>(goal: Goal<'a, D>, query: Q) -> Vec<Q::Reified>
where
    D: Domain<'a> + 'a,
    Q: ReifyIn<'a, D> + 'a,
{
    let state = goal.apply(State::new());
    state.query(query).collect()
}

/// Test helper for ensuring that goals work no matter the order they are
/// applied.
///
/// When building lower level goals, it can be easy to make mistakes where
/// something appears to work fine but breaks when you reorder the goals. This
/// is especially a problem with [projection goals](crate::goal::project).
///
/// This function takes a `Vec<Goal<_>>`, a [Query](crate::Query) and a `Vec`
/// containing the expected values. It will try every permutation of the goals
/// (wrapped in an [`all` goal](crate::goal::all!)) and panic if any of the
/// results vary.
///
/// # Example
/// ```
/// use canrun::{Goal, var, unify, assert_1, util};
/// use canrun::example::I32;
///
/// let x = var();
/// let goals: Vec<Goal<I32>> = vec![
///     unify(2, x),
///     assert_1(x, |x| *x > 1),
/// ];
///
/// util::assert_permutations_resolve_to(goals, x, vec![2]);
/// ```
pub fn assert_permutations_resolve_to<'a, D, Q>(
    goals: Vec<Goal<'a, D>>,
    query: Q,
    expected: Vec<Q::Reified>,
) where
    D: Domain<'a> + Debug + 'a,
    Q: ReifyIn<'a, D> + Clone + 'a,
    Q::Reified: PartialEq + Clone + Debug,
{
    for permutation in all_permutations(goals) {
        let results: Vec<Q::Reified> = goals_resolve_to(&permutation, query.clone());
        if !expected
            .clone()
            .into_iter()
            .permutations(expected.len())
            .any(|e: Vec<Q::Reified>| e == results)
        {
            dbg!(permutation, results, expected);
            panic!("The permutation of the goals printed above failed!");
        }
    }
}
