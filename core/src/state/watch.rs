use crate::value::{LVar, LVarId};

#[derive(Debug)]
pub struct WatchList(pub(crate) Vec<LVarId>);

#[derive(Debug)]
pub enum Watch<State> {
    Done(Option<State>),
    Waiting(State, WatchList),
}

impl<S> Watch<S> {
    pub fn done(state: Option<S>) -> Self {
        Watch::Done(state)
    }
    pub fn watch<T>(state: S, var: LVar<T>) -> Watch<S> {
        Watch::Waiting(state, WatchList(vec![var.id]))
    }
    pub fn and<T>(self, var: LVar<T>) -> Watch<S> {
        match self {
            Watch::Done(Some(state)) => Watch::watch(state, var),
            Watch::Done(None) => self,
            Watch::Waiting(state, mut list) => {
                list.0.push(var.id);
                Watch::Waiting(state, list)
            }
        }
    }
}
