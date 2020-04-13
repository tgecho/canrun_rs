use crate::{DomainType, ResolvedState, Val};

/// Extract fully resolved `T` from a [`Val<T>`](crate::value::Val).
///
/// Used by [query](crate::query) to ensure that result values are fully and
/// recursively resolved.
pub trait ReifyVal<'a, D>: Sized {
    /// The "concrete" type that a `Val<Self>` reifies to.
    type Reified;

    /// Extract a reified `Self` from a [`Val<Self>`](crate::value::Val).
    ///
    /// # Examples:
    /// Simple values are cloned (since the `Val` stores in an [Rc](std::rc::Rc)
    /// internally).
    /// ```
    /// use canrun::{Val, val, var, ReifyVal, IterResolved, State, ResolvedState};
    /// use canrun::domains::example::{I32, VecI32};
    /// State::new()
    ///     .iter_resolved()
    ///     .for_each(|state: ResolvedState<I32>| {
    ///         let x = val!(1);
    ///         assert_eq!(state.reify(&x), Some(1));
    ///     });
    /// ```
    /// Structures containing additional `Val`s should be recursively reified.
    /// ```
    /// # use canrun::{Val, val, var, ReifyVal, IterResolved, State, ResolvedState};
    /// # use canrun::domains::example::{I32, VecI32};
    /// State::new()
    ///     .iter_resolved()
    ///     .for_each(|state: ResolvedState<VecI32>| {
    ///         let x = val!(vec![val!(1), val!(2)]);
    ///         assert_eq!(state.reify(&x), Some(vec![1, 2]));
    ///     });
    /// ```
    /// Returns `None` if the [`Val`] is unresolved.
    /// ```
    /// # use canrun::{Val, val, var, ReifyVal, IterResolved, State, ResolvedState};
    /// # use canrun::domains::example::{I32, VecI32};
    /// State::new()
    ///     .iter_resolved()
    ///     .for_each(|state: ResolvedState<I32>| {
    ///         let x: Val<i32> = val!(var());
    ///         assert_eq!(state.reify(&x), None);
    ///     });
    /// ```
    /// Also returns `None` if `Self` is a structure containing any unresolved
    /// `Val`s.
    /// ```
    /// # use canrun::{Val, val, var, ReifyVal, IterResolved, State, ResolvedState};
    /// # use canrun::domains::example::{I32, VecI32};
    /// State::new()
    ///     .iter_resolved()
    ///     .for_each(|state: ResolvedState<VecI32>| {
    ///         let x: Val<i32> = val!(var());
    ///         let y = val!(vec![x, val!(2)]);
    ///         assert_eq!(state.reify(&y), None);
    ///     });
    /// ```
    fn reify_in(&self, state: &ResolvedState<D>) -> Option<Self::Reified>
    where
        D: DomainType<'a, Self>;
}

impl<'a, D: DomainType<'a, Self>> ReifyVal<'a, D> for i32 {
    type Reified = i32;
    fn reify_in(&self, _: &ResolvedState<D>) -> Option<Self::Reified> {
        Some(*self)
    }
}

impl<'a, D, T> ReifyVal<'a, D> for Vec<Val<T>>
where
    T: ReifyVal<'a, D>,
    D: DomainType<'a, Self> + DomainType<'a, T> + 'a,
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
    use canrun::{val, var, IterResolved, ResolvedState, State, Val};

    #[test]
    fn reify_var() {
        let x: Val<i32> = val!(var());
        State::new()
            .iter_resolved()
            .for_each(|state: ResolvedState<I32>| {
                assert_eq!(state.reify(&x), None);
            });
    }

    #[test]
    fn reify_resolved() {
        let x = val!(1);
        State::new()
            .iter_resolved()
            .for_each(|state: ResolvedState<I32>| {
                assert_eq!(state.reify(&x), Some(1));
            });
    }

    #[test]
    fn reify_vec() {
        let x = val!(vec![val!(1), val!(2)]);
        State::new()
            .iter_resolved()
            .for_each(|state: ResolvedState<VecI32>| {
                assert_eq!(state.reify(&x), Some(vec![1, 2]));
            });
    }
}
