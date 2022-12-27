use crate::{core::State, goals::Goal};
use std::iter::{empty, once};

pub type StateIter = Box<dyn Iterator<Item = State>>;

pub trait StateIterator: 'static {
    fn into_states(self) -> StateIter;
}

impl StateIterator for State {
    fn into_states(mut self) -> StateIter {
        let fork = self.forks.pop_front();
        match fork {
            None => Box::new(once(self)),
            Some(fork) => Box::new(fork.fork(&self).flat_map(StateIterator::into_states)),
        }
    }
}

impl StateIterator for Option<State> {
    fn into_states(self) -> StateIter {
        match self {
            None => Box::new(empty()),
            Some(s) => s.into_states(),
        }
    }
}

impl<G: Goal> StateIterator for G {
    fn into_states(self) -> StateIter {
        self.apply(State::new()).into_states()
    }
}
