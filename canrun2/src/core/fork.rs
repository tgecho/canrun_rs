use super::{State, StateIter};

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
