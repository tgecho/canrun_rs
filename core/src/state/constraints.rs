use crate::value::{LVar, LVarId};

#[derive(Debug)]
pub struct WatchList(pub(crate) Vec<LVarId>);

#[derive(Debug)]
pub enum Constraint<State> {
    Done(Option<State>),
    Waiting(State, WatchList),
}

impl<S> Constraint<S> {
    pub fn done(state: Option<S>) -> Self {
        Constraint::Done(state)
    }
    pub fn on_1<A>(state: S, a: LVar<A>) -> Constraint<S> {
        Constraint::Waiting(state, WatchList(vec![a.id]))
    }
    pub fn on_2<A, B>(state: S, a: LVar<A>, b: LVar<B>) -> Constraint<S> {
        Constraint::Waiting(state, WatchList(vec![a.id, b.id]))
    }
}
