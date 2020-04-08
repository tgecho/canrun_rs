use super::resolved::ResolvedState;
use super::State;
use crate::domains::Domain;

/// An Iterator of [ResolvedStates](crate::state::ResolvedStates).
///
/// Typically obtained through the [.iter_resolved()](IterResolved::iter_resolved()) trait.
pub type ResolvedStateIter<'s, D> = Box<dyn Iterator<Item = ResolvedState<D>> + 's>;

/// Iterate over [ResolvedStates](crate::state::ResolvedState).
///
/// This trait is implemented on the typical values that contain or represent an
/// open state, such as [`Goal`](crate::goal::Goal) and of course
/// [`State`](crate::state::State) itself.
pub trait IterResolved<'a, D: Domain<'a> + 'a> {
    /// Get an iterator of all valid, [resolved
    /// states](crate::state::ResolvedState) that can be derived.
    ///
    /// Typically used indirectly through the
    /// [Queryable](crate::query::Queryable) interface.
    ///
    /// This will iterate through all pending
    /// [forks](crate::state::State::fork()), discarding any that fail. Any
    /// unsatisfied [constraints](crate::state::State::constrain()) will also
    /// cause a potential resolved state to fail.
    ///
    /// # Example:
    /// ```
    /// use canrun::{State, ResolvedState, IterResolved, val, var};
    /// use canrun::domains::example::I32;
    ///
    /// let x = var();
    ///
    /// let state = State::new()
    ///     .unify(&val!(x), &val!(1));
    /// let results: Vec<ResolvedState<I32>> = state.iter_resolved().collect();
    /// ```
    fn iter_resolved(self) -> ResolvedStateIter<'a, D>;
}

impl<'a, D: Domain<'a> + 'a> IterResolved<'a, D> for State<'a, D> {
    fn iter_resolved(self) -> ResolvedStateIter<'a, D> {
        Box::new(self.iter_forks().filter_map(|s: State<'a, D>| {
            if s.constraints.is_empty() {
                Some(ResolvedState { domain: s.domain })
            } else {
                None
            }
        }))
    }
}

impl<'a, D: Domain<'a> + 'a> IterResolved<'a, D> for Option<State<'a, D>> {
    fn iter_resolved(self) -> ResolvedStateIter<'a, D> {
        Box::new(self.into_iter().flat_map(State::iter_resolved))
    }
}
