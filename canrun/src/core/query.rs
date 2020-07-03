use crate::domains::Domain;
use crate::state::IterResolved;
use crate::ReifyIn;

/// Derive [reified](crate::value::ReifyIn) [values](crate::value) potential
/// resolved states.
///
/// [Query] is implemented for [`Goals`](crate::Goal) and
/// [`States`](crate::State) (and other types), meaning you can call
/// [`.query()`](Query::query()) on them with any type that implements
/// [ReifyIn](crate::value::ReifyIn) for a matching [Domain].
///
/// This is a convenient wrapper around the pattern of iterating over a sequence
/// of [`ResolvedStates`](crate::ResolvedState) and calling
/// [`state.reify(query)`](crate::ResolvedState::reify()) and returning only the
/// valid, fully resolved results. Query is implemented on a variety of
/// [`State`](crate::State) related types, allowing it to be used in many
/// contexts.
///
/// A blanket impl covers anything that implements [`IterResolved`], so many
/// types including [`Goal`](crate::Goal) and [`State`](crate::State) are
/// queryable.
pub trait Query<'a, D: Domain<'a> + 'a> {
    /// Get [reified](crate::value::ReifyIn) results from an iterator of
    /// [`ResolvedStates`](crate::ResolvedState).
    ///
    /// # Examples:
    ///
    /// ## Goals
    /// ```
    /// use canrun::{Goal, unify, var, State};
    /// use canrun::example::I32;
    ///
    /// let x = var();
    /// let goal: Goal<I32> = unify(x, 1);
    /// let result: Vec<_> = goal.query(x).collect();
    /// assert_eq!(result, vec![1])
    /// ```
    ///
    /// ## States
    /// ```
    /// use canrun::{State, IterResolved, Query, val, var};
    /// use canrun::example::I32;
    /// let x = var();
    ///
    /// let state: State<I32> = State::new();
    /// let result: Vec<_> = state.query(x).collect();
    /// assert_eq!(result, vec![]) // Nothing has been added to the State
    /// ```
    ///
    /// ### `Option<State<D>>`
    /// Note that most of the lower level [`State`](crate::State) update methods
    /// return an `Option<State<D>>`. Since [`IterResolved`] is implemented for
    /// this type, Query is as well!
    /// ```
    /// # use canrun::{State, IterResolved, Query, val, var};
    /// # use canrun::example::I32;
    /// # let x = var();
    /// let state: Option<State<I32>> = State::new().unify(&val!(x), &val!(1));
    /// let result: Vec<_> = state.query(x).collect();
    /// assert_eq!(result, vec![1])
    /// ```
    fn query<Q>(self, query: Q) -> Box<dyn Iterator<Item = Q::Reified> + 'a>
    where
        Q: ReifyIn<'a, D> + 'a;
}

impl<'a, D: Domain<'a> + 'a, S: IterResolved<'a, D>> Query<'a, D> for S {
    fn query<Q>(self, query: Q) -> Box<dyn Iterator<Item = Q::Reified> + 'a>
    where
        Q: ReifyIn<'a, D> + 'a,
    {
        Box::new(
            self.iter_resolved()
                .filter_map(move |resolved| query.reify_in(&resolved)),
        )
    }
}
