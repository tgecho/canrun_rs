use super::Query;
use crate::domains::DomainType;
use crate::state::ResolvedState;
use crate::value::{LVar, ReifyVal, Val};

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
impl<'a, D, T, R> Query<'a, D> for LVar<T>
where
    D: DomainType<'a, T> + 'a,
    Val<T>: ReifyVal<Reified = R>,
{
    type Result = R;
    fn query_in(&self, state: ResolvedState<D>) -> Option<Self::Result> {
        state.get(*self)
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
impl<'a, D, T1, R1, T2, R2> Query<'a, D> for (LVar<T1>, LVar<T2>)
where
    D: DomainType<'a, T1> + DomainType<'a, T2> + 'a,
    Val<T1>: ReifyVal<Reified = R1>,
    Val<T2>: ReifyVal<Reified = R2>,
{
    type Result = (R1, R2);
    fn query_in(&self, state: ResolvedState<D>) -> Option<Self::Result> {
        Some((state.get(self.0)?, state.get(self.1)?))
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
impl<'a, D, T1, R1, T2, R2, T3, R3> Query<'a, D> for (LVar<T1>, LVar<T2>, LVar<T3>)
where
    D: DomainType<'a, T1> + DomainType<'a, T2> + DomainType<'a, T3> + 'a,
    Val<T1>: ReifyVal<Reified = R1>,
    Val<T2>: ReifyVal<Reified = R2>,
    Val<T3>: ReifyVal<Reified = R3>,
{
    type Result = (R1, R2, R3);
    fn query_in(&self, state: ResolvedState<D>) -> Option<Self::Result> {
        Some((state.get(self.0)?, state.get(self.1)?, state.get(self.2)?))
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
impl<'a, D, T1, R1, T2, R2, T3, R3, T4, R4> Query<'a, D>
    for (LVar<T1>, LVar<T2>, LVar<T3>, LVar<T4>)
where
    D: DomainType<'a, T1> + DomainType<'a, T2> + DomainType<'a, T3> + DomainType<'a, T4> + 'a,
    Val<T1>: ReifyVal<Reified = R1>,
    Val<T2>: ReifyVal<Reified = R2>,
    Val<T3>: ReifyVal<Reified = R3>,
    Val<T4>: ReifyVal<Reified = R4>,
{
    type Result = (R1, R2, R3, R4);
    fn query_in(&self, state: ResolvedState<D>) -> Option<Self::Result> {
        Some((
            state.get(self.0)?,
            state.get(self.1)?,
            state.get(self.2)?,
            state.get(self.3)?,
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
impl<'a, D, T1, R1, T2, R2, T3, R3, T4, R4, T5, R5> Query<'a, D>
    for (LVar<T1>, LVar<T2>, LVar<T3>, LVar<T4>, LVar<T5>)
where
    D: DomainType<'a, T1>
        + DomainType<'a, T2>
        + DomainType<'a, T3>
        + DomainType<'a, T4>
        + DomainType<'a, T5>
        + 'a,
    Val<T1>: ReifyVal<Reified = R1>,
    Val<T2>: ReifyVal<Reified = R2>,
    Val<T3>: ReifyVal<Reified = R3>,
    Val<T4>: ReifyVal<Reified = R4>,
    Val<T5>: ReifyVal<Reified = R5>,
{
    type Result = (R1, R2, R3, R4, R5);
    fn query_in(&self, state: ResolvedState<D>) -> Option<Self::Result> {
        Some((
            state.get(self.0)?,
            state.get(self.1)?,
            state.get(self.2)?,
            state.get(self.3)?,
            state.get(self.4)?,
        ))
    }
}
