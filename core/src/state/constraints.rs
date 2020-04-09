use crate::value::{LVar, LVarId};

/// A set of variables to watch on behalf of a [constraint function](crate::state::State::constrain()).
///
/// Created with the `[Constraint]::on_{n}()` functions.
// This is a private field because we actually strip the LVar type param for internal storage/lookup.
#[derive(Debug)]
pub struct WatchList(pub(crate) Vec<LVarId>);

/// The return value of a [constraint function](crate::state::State::constrain()).
#[derive(Debug)]
pub enum Constraint<State> {
    /// The constraint is satisfied.
    Done(Option<State>),
    /// The constraint needs to be re-run when any of the specified variables are resolved.
    Waiting(State, WatchList),
}

impl<S> Constraint<S> {
    /// Indicate that the constraint is satisfied.
    ///
    /// The returned state may be updated as needed.
    pub fn done(state: Option<S>) -> Self {
        Constraint::Done(state)
    }

    /// Indicate that the constraint needs to be re-run when the specified variable is resolved.
    ///
    /// The returned [`State`](crate::state::State) must _NOT_ be modified.
    pub fn on_1<A>(state: S, a: LVar<A>) -> Constraint<S> {
        Constraint::Waiting(state, WatchList(vec![a.id]))
    }

    /// Indicate that the constraint needs to be re-run when either of the the specified variables are resolved.
    ///
    /// The returned [`State`](crate::state::State) must _NOT_ be modified.
    pub fn on_2<A, B>(state: S, a: LVar<A>, b: LVar<B>) -> Constraint<S> {
        Constraint::Waiting(state, WatchList(vec![a.id, b.id]))
    }
}
