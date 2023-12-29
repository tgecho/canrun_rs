use super::{Reify, StateIterator};

/**
Derive [reified](crate::core::Reify) [values](crate::Value) potential
resolved states.

[Query] is implemented for [`Goals`](crate::goals) and
[`States`](crate::State), meaning you can call
[`.query()`](Query::query()) on them with any type that implements
[`Reify`].

This is a convenient wrapper around the common pattern of obtaining a
[`StateIter`](crate::core::StateIter), making sure each resulting state is
[`.ready()`](crate::State::ready), and calling
[`.reify(query)`](crate::core::ReadyState::reify()) to return only the
valid, fully resolved results. Query is implemented on a variety of
[`State`](crate::State) related types, allowing it to be used in many
contexts.

If a state is not ready (meaning it has open forks and/or constraints still
waiting for variables to be resolved) it will not be returned by the query.

A blanket impl covers anything that implements [`StateIterator`], so many
types including [`Goal`](crate::goals) and [`State`](crate::State) are
queryable.
*/
pub trait Query {
    /**
    Get [reified](crate::core::Reify) results from things that can produce
    [`StateIter`](crate::core::StateIter)s.

    This will call [`State::ready()`](crate::State::ready) internally, so results will not be returned
    from states with unresolved constraints.

    # Examples:

    ## Goals
    ```
    use canrun::{Query, LVar};
    use canrun::goals::unify;

    let x = LVar::new();
    let goal = unify(x, 1);
    let result: Vec<_> = goal.query(x).collect();
    assert_eq!(result, vec![1])
    ```

    ### `State` and `Option<State>`
    Most of the lower level [`State`](crate::State) update methods
    return an `Option<State>`. Since [`StateIterator`] is implemented for
    this type, Query is as well!
    ```
    # use canrun::{State, StateIterator, Query, Value};
    # let x = Value::var();
    let state: Option<State> = State::new().unify(&x, &Value::new(1));
    let result: Vec<_> = state.query(x).collect();
    assert_eq!(result, vec![1])
    ```
    */
    fn query<Q: Reify>(self, query: Q) -> impl Iterator<Item = Q::Reified>;
}

impl<S: StateIterator> Query for S {
    fn query<Q: Reify>(self, query: Q) -> impl Iterator<Item = Q::Reified> {
        Box::new(
            self.into_states()
                .filter_map(move |s| query.reify_in(&s.ready()?)),
        )
    }
}
