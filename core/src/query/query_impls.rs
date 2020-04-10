use super::Query;
use crate::domains::DomainType;
use crate::state::ResolvedState;
use crate::value::LVar;

/// Query for a single [`LVar`](crate::value::LVar)
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
impl<'a, D, T> Query<'a, D> for LVar<T>
where
    D: DomainType<'a, T> + 'a,
    T: Clone + 'a,
{
    type Result = T;
    fn query_in(&self, state: ResolvedState<D>) -> Option<Self::Result> {
        state.get(*self).ok().cloned()
    }
}

/// Query for a tuple of two [`LVars`](crate::value::LVar)
///
/// # Example:
/// ```
/// use canrun::{Goal, both, unify, var};
/// use canrun::domains::example::I32;
///
/// let (x, y) = (var(), var());
/// let goal: Goal<I32> = both(unify(x, 1), unify(y, 2));
/// let query = (x, y);
/// let result: Vec<_> = goal.query(query).collect();
/// assert_eq!(result, vec![(1, 2)])
/// ```
impl<'a, D, T1, T2> Query<'a, D> for (LVar<T1>, LVar<T2>)
where
    D: DomainType<'a, T1> + DomainType<'a, T2> + 'a,
    T1: Clone + 'a,
    T2: Clone + 'a,
{
    type Result = (T1, T2);
    fn query_in(&self, state: ResolvedState<D>) -> Option<Self::Result> {
        Some((
            state.get(self.0).ok().cloned()?,
            state.get(self.1).ok().cloned()?,
        ))
    }
}

/// Query for a tuple of three [`LVars`](crate::value::LVar)
///
/// # Example:
/// ```
/// use canrun::{Goal, all, unify, var};
/// use canrun::domains::example::I32;
///
/// let (x, y, z) = (var(), var(), var());
/// let goal: Goal<I32> = all![unify(x, 1), unify(y, 2), unify(z, 3)];
/// let query = (x, y, z);
/// let result: Vec<_> = goal.query(query).collect();
/// assert_eq!(result, vec![(1, 2, 3)])
/// ```
impl<'a, D, T1, T2, T3> Query<'a, D> for (LVar<T1>, LVar<T2>, LVar<T3>)
where
    D: DomainType<'a, T1> + DomainType<'a, T2> + DomainType<'a, T3> + 'a,
    T1: Clone + 'a,
    T2: Clone + 'a,
    T3: Clone + 'a,
{
    type Result = (T1, T2, T3);
    fn query_in(&self, state: ResolvedState<D>) -> Option<Self::Result> {
        Some((
            state.get(self.0).ok().cloned()?,
            state.get(self.1).ok().cloned()?,
            state.get(self.2).ok().cloned()?,
        ))
    }
}

/// Query for a tuple of four [`LVars`](crate::value::LVar)
///
/// # Example:
/// ```
/// use canrun::{Goal, all, unify, var};
/// use canrun::domains::example::I32;
///
/// let (w, x, y, z) = (var(), var(), var(), var());
/// let goal: Goal<I32> = all![unify(w, 0), unify(x, 1), unify(y, 2), unify(z, 3)];
/// let query = (w, x, y, z);
/// let result: Vec<_> = goal.query(query).collect();
/// assert_eq!(result, vec![(0, 1, 2, 3)])
/// ```
impl<'a, D, T1, T2, T3, T4> Query<'a, D> for (LVar<T1>, LVar<T2>, LVar<T3>, LVar<T4>)
where
    D: DomainType<'a, T1> + DomainType<'a, T2> + DomainType<'a, T3> + DomainType<'a, T4> + 'a,
    T1: Clone + 'a,
    T2: Clone + 'a,
    T3: Clone + 'a,
    T4: Clone + 'a,
{
    type Result = (T1, T2, T3, T4);
    fn query_in(&self, state: ResolvedState<D>) -> Option<Self::Result> {
        Some((
            state.get(self.0).ok().cloned()?,
            state.get(self.1).ok().cloned()?,
            state.get(self.2).ok().cloned()?,
            state.get(self.3).ok().cloned()?,
        ))
    }
}

/// Query for a tuple of five [`LVars`](crate::value::LVar)
///
/// # Example:
/// ```
/// use canrun::{Goal, all, unify, var};
/// use canrun::domains::example::I32;
///
/// let (v, w, x, y, z) = (var(), var(), var(), var(), var());
/// let goal: Goal<I32> = all![unify(v, -1), unify(w, 0), unify(x, 1), unify(y, 2), unify(z, 3)];
/// let query = (v, w, x, y, z);
/// let result: Vec<_> = goal.query(query).collect();
/// assert_eq!(result, vec![(-1, 0, 1, 2, 3)])
/// ```
impl<'a, D, T1, T2, T3, T4, T5> Query<'a, D> for (LVar<T1>, LVar<T2>, LVar<T3>, LVar<T4>, LVar<T5>)
where
    D: DomainType<'a, T1>
        + DomainType<'a, T2>
        + DomainType<'a, T3>
        + DomainType<'a, T4>
        + DomainType<'a, T5>
        + 'a,
    T1: Clone + 'a,
    T2: Clone + 'a,
    T3: Clone + 'a,
    T4: Clone + 'a,
    T5: Clone + 'a,
{
    type Result = (T1, T2, T3, T4, T5);
    fn query_in(&self, state: ResolvedState<D>) -> Option<Self::Result> {
        Some((
            state.get(self.0).ok().cloned()?,
            state.get(self.1).ok().cloned()?,
            state.get(self.2).ok().cloned()?,
            state.get(self.3).ok().cloned()?,
            state.get(self.4).ok().cloned()?,
        ))
    }
}
