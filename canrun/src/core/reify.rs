use super::Unify;
use crate::{
    core::{LVar, Value},
    LVarList, ReadyState,
};

/**
Extract a fully resolved `T` from a [`Value<T>`](Value) associated with a [`State`](crate::State).

Used by [Query](crate::Query) to ensure that result values are fully and
recursively resolved.
*/
pub trait Reify {
    /// The "concrete" type that `Self` reifies to.
    type Reified;

    /// Extract a reified `Self` from a compatible [`State`](crate::State). This trait is usually
    /// used indirectly through the [`Query`](crate::Query) trait.
    ///
    /// # Examples:
    /// Simple values are typically copied or cloned (since the `Value` uses
    /// an [Rc](std::rc::Rc) internally).
    /// ```
    /// use canrun::{Value, Reify, StateIterator, State};
    /// State::new()
    /// .into_states()
    /// .filter_map(|s| s.ready())
    /// .for_each(|state| {
    /// let x = Value::new(1);
    /// // This value is already resolved, so we simply get it back.
    /// assert_eq!(x.reify_in(&state), Ok(1));
    /// });
    /// ```
    /// Structures containing additional `Value`s should be recursively reified.
    /// `Reify` is implemented for several tuple sizes to allow easy querying of
    /// multiple `Value`s.
    /// ```
    /// # use canrun::{Value, Reify, StateIterator, State};
    /// State::new()
    /// .into_states()
    /// .filter_map(|s| s.ready())
    /// .for_each(|state| {
    /// let x = (Value::new(1), Value::new(2));
    /// assert_eq!(x.reify_in(&state), Ok((1, 2)));
    /// });
    /// ```
    /// Returns `None` if the [`Value`] is unresolved. Note that this does not
    /// currently do anything to help you if there are pending forks or
    /// constraints that *could* affect resolution.
    /// ```
    /// # use canrun::{Value, Reify, StateIterator, State};
    /// State::new()
    /// .into_states()
    /// .filter_map(|s| s.ready())
    /// .for_each(|state| {
    /// let x: Value<usize> = Value::var();
    /// assert!(x.reify_in(&state).is_err());
    /// });
    /// ```
    /// Also returns `None` if `Self` is a structure containing any unresolved
    /// `Value`s.
    /// ```
    /// # use canrun::{Value, Reify, StateIterator, State};
    /// State::new()
    /// .into_states()
    /// .filter_map(|s| s.ready())
    /// .for_each(|state| {
    /// let x: Value<i32> = Value::var();
    /// let y = (x, Value::new(2));
    /// assert!(y.reify_in(&state).is_err());
    /// });
    /// ```
    /// # Errors
    /// Will return an `Err(LVarList)` with the first unresolveable `Value<T>` encountered.
    fn reify_in(&self, state: &ReadyState) -> Result<Self::Reified, LVarList>;
}

impl<T: Unify + Reify> Reify for Value<T> {
    type Reified = T::Reified;
    fn reify_in(&self, state: &ReadyState) -> Result<Self::Reified, LVarList> {
        state.resolve(self).resolved()?.reify_in(state)
    }
}

impl<T: Unify + Reify> Reify for LVar<T> {
    type Reified = T::Reified;
    fn reify_in(&self, state: &ReadyState) -> Result<Self::Reified, LVarList> {
        state.resolve(&self.into()).resolved()?.reify_in(state)
    }
}

macro_rules! impl_reify_copy {
    ($($type:ty),+) => {
        $(
            impl Reify for $type {
                type Reified = $type;
                fn reify_in(&self, _: &ReadyState) -> Result<$type, LVarList> {
                    Ok(*self)
                }
            }
        )+
    }
}
macro_rules! impl_reify_clone {
    ($($type:ty),+) => {
        $(
            impl Reify for $type {
                type Reified = $type;
                fn reify_in(&self, _: &ReadyState) -> Result<$type, LVarList> {
                    Ok(self.clone())
                }
            }
        )+
    }
}

impl_reify_copy!(i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64);
impl_reify_copy!(&'static str, bool, char);
impl_reify_clone!(String);

#[cfg(test)]
mod tests {
    use crate::{goals::assert_1, Goal, LVar, LVarList, Reify, State, Value};

    #[test]
    fn reify_copy() {
        let resolved = Value::new(1);
        assert_eq!(resolved.reify_in(&State::new().ready().unwrap()), Ok(1));
    }

    #[test]
    fn reify_clone() {
        let resolved = Value::new("foo".to_string());
        assert_eq!(
            resolved.reify_in(&State::new().ready().unwrap()),
            Ok("foo".to_string())
        );
    }

    #[test]
    fn reify_unresolved() {
        let resolved = Value::new("foo".to_string());
        let unresolved: LVar<String> = LVar::new();
        assert_eq!(
            (resolved, unresolved).reify_in(&State::new().ready().unwrap()),
            Err(LVarList::one(&unresolved))
        );
    }
}
