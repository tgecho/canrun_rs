use super::super::domain::Domain;
use super::super::goal::Goal;
use itertools::Itertools;

pub(crate) fn all_permutations<'a, D: Domain + 'a>(
    goals: Vec<Goal<'a, D>>,
) -> impl Iterator<Item = Vec<Goal<'a, D>>> + 'a {
    let goals_len = goals.len();
    goals.into_iter().permutations(goals_len)
}

