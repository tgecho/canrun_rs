use crate::{
    core::{AnyVal, Unify, Value, VarId},
    resolve_any, Reify,
};

/**
Derives from a [`State`](crate::State) that has been confirmed to have no open forks or constraints.
A `ReadyState` can have [`Value`]s [reified](ReadyState::reify).
 */
#[derive(Clone)]
pub struct ReadyState {
    pub(crate) values: im_rc::HashMap<VarId, AnyVal>,
}

impl ReadyState {
    pub(crate) fn new(values: im_rc::HashMap<VarId, AnyVal>) -> Self {
        ReadyState { values }
    }

    /// * Recursively resolve a [`Value`] as far as the currently known variable
    /// bindings allow.
    ///
    /// See [`crate::State::resolve`] for more details.
    ///
    /// # Panics
    ///
    /// This will panic if the stored [`Value`] resolves with a different `T`
    /// than what is passed in. This shouldn't happen unless the `T` associated
    /// with an [`LVar`] is somehow changed.
    pub fn resolve<T: Unify>(&self, val: &Value<T>) -> Value<T> {
        resolve_any(&self.values, &val.to_anyval())
            .to_value()
            // I think this should be safe, so long as we are careful to only
            // store a var with the correct type internally.
            .expect("AnyVal resolved to unexpected Value<T>")
    }

    /** Attempt to [reify](crate::core::Reify) the value of a [logic
    variable](crate::core::LVar) in a [`ReadyState`].

    # Example:
    ```
    use canrun::{State, StateIterator, Value, LVar};

    let x = LVar::new();

    let state = State::new()
        .unify(&x.into(), &Value::new(1));

    let results: Vec<_> = state.into_states()
        .filter_map(|s| s.ready())
        .map(|resolved| resolved.reify(&x))
        .collect();

    assert_eq!(results, vec![Some(1)]);
    ```
    */
    pub fn reify<T, R>(&self, value: &T) -> Option<R>
    where
        T: Reify<Reified = R>,
    {
        value.reify_in(self)
    }
}
