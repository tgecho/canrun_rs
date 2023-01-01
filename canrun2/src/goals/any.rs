use std::{iter::repeat, rc::Rc};

use super::Goal;
use crate::core::{Fork, State, StateIter};

#[derive(Debug, Clone)]
pub struct Any {
    goals: Vec<Rc<dyn Goal>>,
}

impl<I: Iterator<Item = Rc<dyn Goal>>> From<I> for Any {
    fn from(iter: I) -> Self {
        Any {
            goals: iter.collect(),
        }
    }
}

impl Goal for Any {
    fn apply(&self, state: State) -> Option<State> {
        state.fork(self.clone())
    }
}

impl Fork for Any {
    fn fork(&self, state: &State) -> StateIter {
        let goals = self.goals.clone().into_iter();
        let states = repeat(state.clone());
        Box::new(goals.zip(states).flat_map(|(g, s)| g.apply(s).into_iter()))
    }
}

/// Create a [goal](crate::goals::Goal) that yields a state for every successful
/// sub-goal.
///
/// This is essentially an "OR" operation on a vector of goals. It may yield
/// from zero to as many resolved [states](crate::core::State) as there
/// are sub-goals.
#[macro_export]
macro_rules! any {
    ($($item:expr),* $(,)?) => {
        $crate::goals::any::Any {goals:vec![$(std::rc::Rc::new($item)),*]}
    };
}
pub use any;

#[cfg(test)]
mod tests {
    use crate::{
        core::LVar,
        core::Query,
        goals::{both::both, fail::Fail, unify},
    };

    use super::any;

    #[test]
    fn both_succeed() {
        let x = LVar::new();
        let goal = any![unify(x, 5), unify(x, 7)];
        let result = goal.query(x).collect::<Vec<_>>();
        assert_eq!(result, vec![5, 7]);
    }

    #[test]
    fn one_succeeds() {
        let x = LVar::new();
        let goal = any![unify(x, 5), both(Fail, unify(x, 7))];
        let result = goal.query(x).collect::<Vec<_>>();
        assert_eq!(result, vec![5]);
    }

    #[test]
    fn all_fail() {
        let x = LVar::new();
        let goal = any![both(Fail, unify(x, 5)), both(Fail, unify(x, 7))];
        let result = goal.query(x).collect::<Vec<_>>();
        assert_eq!(result, vec![]);
    }
}
