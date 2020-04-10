use crate::value::Val;

/// Extract fully resolved `T` from a [`Val<T>`](crate::value::Val).
///
/// Used by [query](crate::query) to ensure that result values are fully and
/// recursively resolved.
pub trait ReifyVal {
    /// The "concrete" type that a `Val<T>` reifies to.
    type Reified;

    /// Extract a reified `T` from a [`Val<T>`](crate::value::Val).
    ///
    /// # Examples:
    /// Simple values are cloned (since the `Val` stores in an [Rc](std::rc::Rc)
    /// internally).
    /// ```
    /// use canrun::{Val, val, var, ReifyVal};
    /// let x = val!(1);
    /// assert_eq!(x.reify(), Some(1));
    /// ```
    /// Structures containing additional `Val`s should be recursively reified.
    /// ```
    /// # use canrun::{Val, val, var, ReifyVal};
    /// let x = val!(vec![val!(1), val!(2)]);
    /// assert_eq!(x.reify(), Some(vec![1, 2]));
    /// ```
    /// Returns `None` if the `Val` is unresolved.
    /// ```
    /// # use canrun::{Val, val, var, ReifyVal};
    /// let x: Val<i32> = val!(var());
    /// assert_eq!(x.reify(), None);
    /// ```
    /// Also returns `None` if `T` is a structure containing any unresolved
    /// `Val`s.
    /// ```
    /// # use canrun::{Val, val, var, ReifyVal};
    /// let x: Val<i32> = val!(var());
    /// let y = val!(vec![x, val!(2)]);
    /// assert_eq!(y.reify(), None);
    /// ```
    fn reify(&self) -> Option<Self::Reified>;
}

impl ReifyVal for Val<i32> {
    type Reified = i32;
    fn reify(&self) -> Option<Self::Reified> {
        self.resolved().ok().copied()
    }
}

impl<ValT: ReifyVal> ReifyVal for Val<Vec<ValT>> {
    type Reified = Vec<ValT::Reified>;
    fn reify(&self) -> Option<Self::Reified> {
        let vec = self.resolved().ok()?;
        vec.into_iter().map(|v| v.reify()).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate as canrun;
    use crate::value::{reify_val::ReifyVal, val, var, Val};

    #[test]
    fn reify_var() {
        let x: Val<i32> = val!(var());
        assert_eq!(x.reify(), None);
    }

    #[test]
    fn reify_resolved() {
        let x = val!(1);
        assert_eq!(x.reify(), Some(1));
    }

    #[test]
    fn reify_vec() {
        let x = val!(vec![val!(1), val!(2)]);
        assert_eq!(x.reify(), Some(vec![1, 2]));
    }
}
