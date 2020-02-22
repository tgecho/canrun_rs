use crate::{all, Can, CanT, Goal, LVar, State};
use itertools::Itertools;

pub(crate) fn all_permutations<'a, T: CanT + 'a>(
    goals: Vec<Goal<'a, T>>,
) -> impl Iterator<Item = Vec<Goal<'a, T>>> + 'a {
    let goals_len = goals.len();
    goals.into_iter().permutations(goals_len)
}

pub(crate) fn resolve_to<'a, T: CanT + 'a>(
    goals: &Vec<Goal<'a, T>>,
    vars: &Vec<LVar>,
) -> Vec<Vec<Can<T>>> {
    all(goals.clone())
        .run(State::new())
        .map(|s| {
            let results = vars.iter().map(|v| s.resolve_var(*v).unwrap());
            results.collect::<Vec<Can<T>>>()
        })
        .collect()
}
