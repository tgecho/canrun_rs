use std::rc::Rc;

use crate::{state_iterator::StateIter, State};

pub trait Fork: 'static {
    /// Given a [`State`], return an iterator of states that result from the
    /// fork operation.
    fn fork(&self, state: &State) -> StateIter;
}

impl<F: 'static> Fork for F
where
    F: Fn(&State) -> StateIter,
{
    fn fork(&self, state: &State) -> StateIter {
        self(state)
    }
}

impl State {
    pub fn fork<F: Fork>(mut self, fork: F) -> Option<Self> {
        self.forks.push_back(Rc::new(fork));
        Some(self)
    }
}