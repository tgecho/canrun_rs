//! Extract [values](crate::value) from the results of resolving potential states.
//!
//! [`Goals`](crate::Goal) and [`States`](crate::State) (and other types) are
//! [Queryable], meaning you can call [`.query()`](Queryable::query()) on them with any type that
//! implements [Query] for a matching [Domain].
//!
//! Queries will typically only return results for states in which the requested
//! values are fully resolved, though this is not a hard requirement. A [Query]
//! is free to define its own requirements about the status or contents of any
//! values in a result.
//!
//! # Example:
//! ```
//! use canrun::{Goal, unify, var, State};
//! use canrun::domains::example::I32;
//!
//! let x = var();
//! let goal: Goal<I32> = unify(x, 1);
//! let result: Vec<_> = goal.query(x).collect();
//! assert_eq!(result, vec![1])
//! ```
use crate::domains::Domain;
use crate::state::{IterResolved, ResolvedState};

mod query_impls;

/// Types that can be queried with the [Query] trait.
///
/// This is a convenient wrapper around the pattern of iterating over a sequence
/// of [`ResolvedStates`](crate::ResolvedState) and applying a [Query], returning
/// only the valid, fully resolved results. Queryable is implemented on a
/// variety of [`State`](crate::State) related types, allowing it to be used in
/// many contexts.
///
/// A blanket impl covers anything that implements [`IterResolved`], so many types
/// including [`Goal`](crate::Goal) and [`State`](crate::State) are queryable.
///
pub trait Queryable<'a, D: Domain<'a> + 'a> {
    /// Applies a [Query] to an iterator of [`ResolvedStates`](crate::ResolvedState) to derive results.
    ///
    /// # Examples:
    ///
    /// ## Goals
    /// ```
    /// use canrun::{Goal, unify, var, State};
    /// use canrun::domains::example::I32;
    ///
    /// let x = var();
    /// let goal: Goal<I32> = unify(x, 1);
    /// let result: Vec<_> = goal.query(x).collect();
    /// assert_eq!(result, vec![1])
    /// ```
    ///
    /// ## States
    /// ```
    /// use canrun::{State, IterResolved, Queryable, val, var};
    /// use canrun::domains::example::I32;
    /// let x = var();
    ///
    /// let state: State<I32> = State::new();
    /// let result: Vec<_> = state.query(x).collect();
    /// assert_eq!(result, vec![]) // Nothing has been added to the State
    /// ```
    ///
    /// ### `Option<State<D>>`
    /// Note that most of the lower level [`State`](crate::State) update methods
    /// return an `Option<State<D>>`. Since [`IterResolved`] is implemented for this
    /// type, Queryable is as well!
    /// ```
    /// # use canrun::{State, IterResolved, Queryable, val, var};
    /// # use canrun::domains::example::I32;
    /// # let x = var();
    /// let state: Option<State<I32>> = State::new().unify(&val!(x), &val!(1));
    /// let result: Vec<_> = state.query(x).collect();
    /// assert_eq!(result, vec![1])
    /// ```
    fn query<Q>(self, query: Q) -> Box<dyn Iterator<Item = Q::Result> + 'a>
    where
        Q: Query<'a, D> + 'a;
}

impl<'a, D: Domain<'a> + 'a, S: IterResolved<'a, D>> Queryable<'a, D> for S {
    fn query<Q>(self, query: Q) -> Box<dyn Iterator<Item = Q::Result> + 'a>
    where
        Q: Query<'a, D> + 'a,
    {
        Box::new(
            self.iter_resolved()
                .filter_map(move |resolved| query.query_in(resolved)),
        )
    }
}

/// Query for resolved values in a
/// [`ResolvedState`](crate::state::ResolvedState).
///
/// Manually extracting resolved values from a set of states can be tedious for
/// simple cases. Every var is not guaranteed to be fully resolved, and we may
/// need to search through a few potential states to find one with all of the
/// values we're looking for. Types implementing the Query trait are typically
/// passed as an argument to [Queryable] trait's [`.query()`](Queryable::query())
/// method.
pub trait Query<'a, D: Domain<'a> + 'a> {
    /// The type returned by the [`.query_in()`](Query::query_in()) function.
    type Result;

    /// Attempt to extract a [query result](Query::Result) from a
    /// [`ResolvedState`].
    ///
    /// # Example:
    /// ```
    /// use canrun::{Goal, unify, var};
    /// use canrun::domains::example::I32;
    ///
    /// let x = var();
    /// let goal: Goal<I32> = unify(x, 1);
    /// let query = x;
    /// let result: Vec<_> = goal.query(query).collect();
    /// assert_eq!(result, vec![1])
    /// ```
    fn query_in(&self, state: ResolvedState<D>) -> Option<Self::Result>;
}
