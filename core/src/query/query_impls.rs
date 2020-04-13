use super::Query;
use crate::domains::DomainType;
use crate::state::ResolvedState;
use crate::value::{LVar, ReifyVal};

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
    T: ReifyVal<'a, D, Reified = R>,
{
    type Result = R;
    fn query_in(&self, state: ResolvedState<D>) -> Option<Self::Result> {
        state.get(*self)
    }
}

macro_rules! impl_tuple_query {
    ($(#[$attr:meta])* $($t:ident => $r:ident,)*) => {
        $(#[$attr])*
        impl<'a, D, $($t, $r),*> Query<'a, D> for ($(LVar<$t>,)*)
        where
            D: $(DomainType<'a, $t> +)* 'a,
            $($t: ReifyVal<'a, D, Reified = $r>),*
        {
            type Result = ($($r,)*);
            fn query_in(&self, state: ResolvedState<D>) -> Option<Self::Result> {
                #![allow(non_snake_case)]
                let ($($t),*) = *self;
                Some(($(state.get($t)?),*))
            }
        }
    };
}

impl_tuple_query! {
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
    Aq => Ar,
    Bq => Br,
}

impl_tuple_query! {
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
    Aq => Ar,
    Bq => Br,
    Cq => Cr,
}

impl_tuple_query! {
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
    Aq => Ar,
    Bq => Br,
    Cq => Cr,
    Dq => Dr,
}

impl_tuple_query! {
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
    Aq => Ar,
    Bq => Br,
    Cq => Cr,
    Dq => Dr,
    Eq => Er,
}
