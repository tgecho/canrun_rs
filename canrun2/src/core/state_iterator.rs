use crate::core::State;
use std::iter::once;

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

impl<S: StateIterator, I: IntoIterator<Item = S> + 'static> StateIterator for I {
    fn into_states(self) -> StateIter {
        Box::new(self.into_iter().flat_map(|s| s.into_states()))
    }
}
