use crate::domains::Domain;
use crate::state::{IterResolved, ResolvedState};

mod query_impls;

/// Applies a [Query] to a set of [ResolvedStates](crate::ResolvedState).
///
/// This is a convenient wrapper around the pattern of iterating over a sequence
/// of [ResolvedStates](crate::ResolvedState) and applying a [Query], returning
/// only the valid, fully resolved results. Queryable is implemented on a
/// variety of [State](crate::State) related types, allowing it to be used in
/// many contexts.
///
/// A blanket impl covers anything that implements [IterResolved], so many types
/// including [Goal](crate::Goal) and [State](crate::State) are Queryable.
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
/// Note that most of the lower level [State](crate::State) update methods
/// return an `Option<State<D>>`. Since [IterResolved] is implemented for this
/// type, Queryable is as well!
/// ```
/// # use canrun::{State, IterResolved, Queryable, val, var};
/// # use canrun::domains::example::I32;
/// # let x = var();
/// let state: Option<State<I32>> = State::new().unify(val!(x), val!(1));
/// let result: Vec<_> = state.query(x).collect();
/// assert_eq!(result, vec![1])
/// ```
pub trait Queryable<'a, D: Domain<'a> + 'a> {
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
/// [ResolvedState](crate::state::ResolvedState).
///
/// Manually extracting resolved values from a set of states can be tedious for
/// simple cases. Every var is not guaranteed to be fully resolved, and we may
/// need to search through a few potential states to find one with all of the
/// values we're looking for. Types implementing the Query trait are typically
/// passed as an argument to [Queryable] trait's [.query()](Queryable::query())
/// method.
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
pub trait Query<'a, D: Domain<'a> + 'a> {
    type Result;
    fn query_in(&self, state: ResolvedState<'a, D>) -> Option<Self::Result>;
}
