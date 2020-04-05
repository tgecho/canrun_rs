pub(super) mod multikeymultivaluemap;

use super::state::State;
use crate::domains::Domain;
use crate::goal::Goal;
use crate::query::{Query, Queryable};
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

pub(crate) fn goals_resolve_to<'a, D, Q>(goals: &Vec<Goal<'a, D>>, query: Q) -> Vec<Q::Result>
where
    D: Domain<'a> + 'a,
    Q: Query<'a, D>,
{
    let goal = Goal::All(goals.clone());
    goal_resolves_to(goal, query)
}

pub(crate) fn goal_resolves_to<'a, D, Q>(goal: Goal<'a, D>, query: Q) -> Vec<Q::Result>
where
    D: Domain<'a> + 'a,
    Q: Query<'a, D>,
{
    let state = goal.apply(State::new());
    state.query(query).collect()
}

pub fn all_permutations_resolve_to<'a, D, Q>(
    goals: Vec<Goal<'a, D>>,
    query: Q,
    expected: Vec<Q::Result>,
) where
    D: Domain<'a> + Debug + 'a,
    Q: Query<'a, D> + Clone,
    Q::Result: PartialEq + Debug,
{
    for permutation in all_permutations(goals) {
        dbg!(&permutation);
        assert_eq!(goals_resolve_to(&permutation, query.clone()), expected);
    }
}
