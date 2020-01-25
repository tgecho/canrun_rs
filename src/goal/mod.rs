use crate::state::State;
use itertools::Itertools;
use std::iter::empty;

pub mod append;
pub mod both;
pub mod either;
pub mod equal;
pub mod lazy;

pub use both::both;
pub use either::either;
pub use equal::equal;
pub use lazy::lazy;

#[derive(Clone)]
pub enum Goal<T: Eq + Clone + 'static> {
    Equal(equal::EqualGoal<T>),
    Both(both::BothGoal<T>),
    Either(either::EitherGoal<T>),
    Lazy(lazy::LazyGoal<T>),
}

type GoalIter<T> = Box<dyn Iterator<Item = State<T>>>;

pub trait Pursue<T: Eq + Clone> {
    fn run<'a>(self, state: &'a State<T>) -> GoalIter<T>;
}

impl<T: Eq + Clone + 'static> Goal<T> {
    pub fn run<'a>(self, state: &'a State<T>) -> GoalIter<T> {
        match self {
            Goal::Equal(goal) => goal.run(state),
            Goal::Both(goal) => goal.run(state),
            Goal::Either(goal) => goal.run(state),
            Goal::Lazy(goal) => goal.run(state),
        }
    }

    pub fn run_in_each(self, mut states: GoalIter<T>) -> GoalIter<T> {
        let first = states.next();
        match first {
            Some(state) => {
                let head = self.clone().run(&state);
                let tail = self.clone().run_in_each(states);
                Box::new(head.interleave(tail))
            }
            None => Box::new(empty()),
        }
    }
}
