use std::rc::Rc;

use crate::{state_iterator::StateIter, State};

pub trait Fork: 'static {
    /// Given a [`State`], return an iterator of states that result from the
    /// fork operation.
    fn fork(&self, state: State) -> StateIter;
}

impl State {
    pub fn fork<F: Fork>(mut self, fork: Rc<F>) -> Option<Self> {
        self.forks.push_back(fork);
        Some(self)
    }
}
