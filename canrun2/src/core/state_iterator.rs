use crate::{core::State, goals::Goal};
use std::iter::{empty, once};

/**
An Iterator of [`State`]s, with all pending [`Fork`](crate::Fork)s applied
and [`Value`](crate::core::Value)s resolved.

Typically obtained with [`Query::query()`](crate::Query::query()) or
[`StateIterator::into_states()`](super::state_iterator::StateIterator::into_states).
*/
pub type StateIter = Box<dyn Iterator<Item = State>>;

/**

This trait is implemented on the typical values that contain or represent an
open state, such as [`Goal`](crate::goals::Goal) and of course
[`State`](crate::core::State) itself.
*/
pub trait StateIterator: 'static {
    /**
    Iterate over [`States`](crate::State) by applying all pending [`Fork`](crate::Fork)s
    and checking [`Constraint`](crate::core::constraints::Constraint)s.
    */
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
