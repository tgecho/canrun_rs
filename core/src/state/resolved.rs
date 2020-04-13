use crate::domains::{Domain, DomainType};
use crate::value::{LVar, ReifyVal, Val};
use std::fmt::Debug;

/// Derived from an open [`State`](crate::state::State), depending on
/// the constraints that have been applied.
///
/// Calling [`.iter_resolved()`](crate::IterResolved::iter_resolved()) is the
/// lowest level way to get an iterator of the possible resolved states, though
/// the [`Query`](crate::query::Query) interface is quite a bit nicer.
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
        match val {
            Val::Var(var) => {
                let resolved = self.domain.values_as_ref().0.get(var);
                match resolved {
                    Some(Val::Var(found)) if found == var => val,
                    Some(found) => self.resolve_val(found),
                    _ => val,
                }
            }
            value => value,
        }
    }

    /// Attempt to [reify](crate::value::ReifyVal) the value of a [logic
    /// variable](crate::value::LVar) in a resolved state.
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
    ///
    /// let results: Vec<_> = state.iter_resolved()
    ///     .map(|resolved: ResolvedState<I32>| resolved.reify_var(x))
    ///     .collect();
    ///
    /// assert_eq!(results, vec![Some(1)]);
    /// ```
    pub fn reify_var<T, R>(&self, var: LVar<T>) -> Option<R>
    where
        D: DomainType<'a, T>,
        T: ReifyVal<'a, D, Reified = R>,
    {
        let val = self.domain.values_as_ref().0.get(&var)?;
        self.reify_val(val)
    }

    /// Attempt to [reify](crate::value::ReifyVal) a [`Val`] in a resolved
    /// state.
    ///
    /// # Example:
    /// ```
    /// use canrun::{State, ResolvedState, IterResolved, val, var, Val};
    /// use canrun::domains::example::I32;
    ///
    /// let x: Val<i32> = val!(var());
    ///
    /// let state = State::new()
    ///     .unify(&x, &val!(1));
    ///
    /// let results: Vec<_> = state.iter_resolved()
    ///     .map(|resolved: ResolvedState<I32>| resolved.reify_val(&x))
    ///     .collect();
    ///
    /// assert_eq!(results, vec![Some(1)]);
    /// ```
    pub fn reify_val<T, R>(&self, val: &Val<T>) -> Option<R>
    where
        D: DomainType<'a, T>,
        T: ReifyVal<'a, D, Reified = R>,
    {
        let resolved = self.resolve_val(val).resolved().ok()?;
        resolved.reify_in(self)
    }
}
