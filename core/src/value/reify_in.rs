use crate::{Domain, DomainType, IntoVal, LVar, ResolvedState, Val};
use std::fmt::Debug;

/// Extract a fully resolved `T` from a [`Val<T>`](crate::value::Val).
///
/// Used by [query](crate::query) to ensure that result values are fully and
/// recursively resolved.
pub trait ReifyIn<'a, D>: Sized {
    /// The "concrete" type that `Self` reifies to.
    type Reified;

    /// Extract a reified `Self` from a compatible
    /// [`ResolvedState`](crate::state::ResolvedState). This trait is usually
    /// used indirectly through the [`Query`](crate::query) trait.
    ///
    /// # Examples:
    /// Simple values are typically copied or cloned (since the `Val` stores in
    /// an [Rc](std::rc::Rc) internally).
    /// ```
    /// use canrun::{Val, val, var, ReifyIn, IterResolved, State, ResolvedState};
    /// use canrun::domains::example::{I32, VecI32};
    /// State::new()
    ///     .iter_resolved()
    ///     .for_each(|state: ResolvedState<I32>| {
    ///         let x = val!(1);
    ///         assert_eq!(x.reify_in(&state), Some(1));
    ///     });
    /// ```
    /// Structures containing additional `Val`s should be recursively reified.
    /// ```
    /// # use canrun::{Val, val, var, ReifyIn, IterResolved, State, ResolvedState};
    /// # use canrun::domains::example::{I32, VecI32};
    /// State::new()
    ///     .iter_resolved()
    ///     .for_each(|state: ResolvedState<VecI32>| {
    ///         let x = val!(vec![val!(1), val!(2)]);
    ///         assert_eq!(x.reify_in(&state), Some(vec![1, 2]));
    ///     });
    /// ```
    /// Returns `None` if the [`Val`] is unresolved.
    /// ```
    /// # use canrun::{Val, val, var, ReifyIn, IterResolved, State, ResolvedState};
    /// # use canrun::domains::example::{I32, VecI32};
    /// State::new()
    ///     .iter_resolved()
    ///     .for_each(|state: ResolvedState<I32>| {
    ///         let x: Val<i32> = val!(var());
    ///         assert_eq!(x.reify_in(&state), None);
    ///     });
    /// ```
    /// Also returns `None` if `Self` is a structure containing any unresolved
    /// `Val`s.
    /// ```
    /// # use canrun::{Val, val, var, ReifyIn, IterResolved, State, ResolvedState};
    /// # use canrun::domains::example::{I32, VecI32};
    /// State::new()
    ///     .iter_resolved()
    ///     .for_each(|state: ResolvedState<VecI32>| {
    ///         let x: Val<i32> = val!(var());
    ///         let y = val!(vec![x, val!(2)]);
    ///         assert_eq!(y.reify_in(&state), None);
    ///     });
    /// ```
    fn reify_in(&self, state: &ResolvedState<D>) -> Option<Self::Reified>;
}

impl<'a, T, D> ReifyIn<'a, D> for LVar<T>
where
    T: ReifyIn<'a, D> + Debug,
    D: DomainType<'a, T> + 'a,
{
    type Reified = T::Reified;
    fn reify_in(&self, state: &ResolvedState<D>) -> Option<Self::Reified> {
        self.into_val().reify_in(state)
    }
}

impl<'a, T, D> ReifyIn<'a, D> for Val<T>
where
    T: ReifyIn<'a, D> + Debug,
    D: DomainType<'a, T> + 'a,
{
    type Reified = T::Reified;
    fn reify_in(&self, state: &ResolvedState<D>) -> Option<Self::Reified> {
        // dbg!("reify Val before", self);
        // dbg!(state.domain.values_as_ref());
        let resolved = state.resolve_val(self).resolved();
        // dbg!("reify Val resolved", resolved);
        let resolved = resolved.ok()?;
        // dbg!("reify Val after");
        resolved.reify_in(state)
    }
}

impl<'a, T, D> ReifyIn<'a, D> for &T
where
    T: ReifyIn<'a, D>,
    D: Domain<'a> + 'a,
{
    type Reified = T::Reified;
    fn reify_in(&self, state: &ResolvedState<D>) -> Option<Self::Reified> {
        (*self).reify_in(state)
    }
}

impl<'a, D: Domain<'a>> ReifyIn<'a, D> for i32 {
    type Reified = i32;
    fn reify_in(&self, _: &ResolvedState<D>) -> Option<Self::Reified> {
        Some(*self)
    }
}

impl<'a, D, T> ReifyIn<'a, D> for Vec<Val<T>>
where
    T: ReifyIn<'a, D> + Debug,
    D: DomainType<'a, T> + 'a,
{
    type Reified = Vec<T::Reified>;
    fn reify_in(&self, state: &ResolvedState<D>) -> Option<Self::Reified> {
        self.into_iter().map(|v: &Val<T>| state.reify(v)).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate as canrun;
    use canrun::domains::example::{VecI32, I32};
    use canrun::{val, var, IterResolved, ReifyIn, ResolvedState, State, Val};

    #[test]
    fn reify_var() {
        let x: Val<i32> = val!(var());
        State::new()
            .iter_resolved()
            .for_each(|state: ResolvedState<I32>| {
                assert_eq!(x.reify_in(&state), None);
            });
    }

    #[test]
    fn reify_resolved() {
        let x = val!(1);
        State::new()
            .iter_resolved()
            .for_each(|state: ResolvedState<I32>| {
                assert_eq!(x.reify_in(&state), Some(1));
            });
    }

    #[test]
    fn reify_vec() {
        let x = val!(vec![val!(1), val!(2)]);
        State::new()
            .iter_resolved()
            .for_each(|state: ResolvedState<VecI32>| {
                assert_eq!(x.reify_in(&state), Some(vec![1, 2]));
            });
    }
}
