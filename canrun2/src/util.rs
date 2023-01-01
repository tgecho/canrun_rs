//! Assorted helpers, especially for testing.

use itertools::Itertools;
use std::{fmt::Debug, rc::Rc};

use crate::{
    core::{Query, Reify},
    goals::{All, Goal},
};

pub(crate) fn all_permutations(
    goals: Vec<Rc<dyn Goal>>,
) -> impl Iterator<Item = Vec<Rc<dyn Goal>>> {
    let goals_len = goals.len();
    goals.into_iter().permutations(goals_len)
}

pub fn assert_permutations_resolve_to<Q>(
    goals: Vec<Rc<dyn Goal>>,
    query: Q,
    expected: Vec<Q::Reified>,
) where
    Q: Reify + Clone,
    Q::Reified: PartialEq + Clone + Debug,
{
    for permutation in all_permutations(goals) {
        let perm_goals = permutation
            .iter()
            .map(|g| Box::new(g.clone()) as Box<dyn Goal>);
        let all_goals: All = perm_goals.collect();
        let results: Vec<Q::Reified> = all_goals.query(query.clone()).collect();
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

pub struct GoalVec(pub Vec<Rc<dyn Goal>>);

impl GoalVec {
    pub fn assert_permutations_resolve_to<Q>(self, query: Q, expected: Vec<Q::Reified>)
    where
        Q: Reify + Clone,
        Q::Reified: PartialEq + Clone + Debug,
    {
        assert_permutations_resolve_to(self.0, query, expected);
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! goal_vec {
    ($($item:expr),* $(,)?) => {
        $crate::util::GoalVec(vec![$(Rc::new($item),)*])
    };
}
pub use goal_vec;
