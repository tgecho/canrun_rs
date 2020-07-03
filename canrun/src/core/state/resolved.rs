use crate::domains::{Domain, DomainType};
use crate::value::{ReifyIn, Val};
use std::fmt::Debug;

/// Derived from an open [`State`](crate::state::State), depending on
/// the constraints that have been applied.
///
/// Calling [`.iter_resolved()`](crate::IterResolved::iter_resolved()) is the
/// lowest level way to get an iterator of the possible resolved states, though
/// the [`Query`](crate::Query) interface is quite a bit nicer.
#[derive(Clone)]
pub struct ResolvedState<D> {
    pub(super) domain: D,
}

impl<'a, D: Domain<'a> + 'a> ResolvedState<D> {
    pub(crate) fn resolve_val<'r, T>(&'r self, val: &'r Val<T>) -> &'r Val<T>
    where
        T: Debug,
        D: DomainType<'a, T>,
    {
        self.domain.resolve(val)
    }

    /// Attempt to [reify](crate::value::ReifyIn) the value of a [logic
    /// variable](crate::value::LVar) in a resolved state.
    ///
    /// # Example:
    /// ```
    /// use canrun::{State, ResolvedState, IterResolved, val, var};
    /// use canrun::example::I32;
    ///
    /// let x = var();
    ///
    /// let state = State::new()
    ///     .unify(&val!(x), &val!(1));
    ///
    /// let results: Vec<_> = state.iter_resolved()
    ///     .map(|resolved: ResolvedState<I32>| resolved.reify(x))
    ///     .collect();
    ///
    /// assert_eq!(results, vec![Some(1)]);
    /// ```
    pub fn reify<T, R>(&self, value: T) -> Option<R>
    where
        D: Domain<'a>,
        T: ReifyIn<'a, D, Reified = R>,
    {
        value.reify_in(self)
    }
}
