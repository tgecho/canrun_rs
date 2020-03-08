use super::super::domain::Domain;
use super::super::goal::Goal;
use super::super::query::{QueryState, StateQuery};
use super::super::state::State;
use itertools::Itertools;
use std::fmt::Debug;

pub(crate) fn all_permutations<'a, D>(
    goals: Vec<Goal<'a, D>>,
) -> impl Iterator<Item = Vec<Goal<'a, D>>> + 'a
where
    D: Domain + 'a,
{
    let goals_len = goals.len();
    goals.into_iter().permutations(goals_len)
}

pub(crate) fn goals_resolve_to<'a, D, Q>(goals: &Vec<Goal<'a, D>>, query: Q) -> Vec<Q::Result>
where
    D: Domain + 'a,
    Q: QueryState<'a, D>,
{
    let goal = Goal::All(goals.clone());
    let state = goal.apply(State::new());
    state.query(query).collect()
}

pub(crate) fn all_permutations_resolve_to<'a, D, Q>(
    goals: Vec<Goal<'a, D>>,
    query: Q,
    expected: Vec<Q::Result>,
) where
    D: Domain + Debug + 'a,
    Q: QueryState<'a, D> + Clone,
    Q::Result: Eq + Debug,
{
    for permutation in all_permutations(goals) {
        dbg!(&permutation);
        assert_eq!(goals_resolve_to(&permutation, query.clone()), expected);
    }
}
