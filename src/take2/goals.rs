mod equal;

use super::domain::DomainType;
use super::state::{State, Unify};

pub type StateIter<'s, State> = Box<dyn Iterator<Item = State> + 's>;

pub trait Goal<'a, T> {
    fn run<S: Unify<'a, T> + 'a>(self, state: S) -> StateIter<'a, S>;
}

pub trait AddGoal<'g, T> {
    fn add_goal<G: Goal<'g, T>>(&mut self, goal: G) -> &mut Self;
}

impl<'g, T, D: DomainType<T>> AddGoal<'g, T> for State<D> {
    fn add_goal<G: Goal<'g, T>>(&mut self, goal: G) -> &mut Self {
        self
    }
}
