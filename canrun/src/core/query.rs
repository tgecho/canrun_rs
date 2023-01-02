use super::{Reify, StateIterator};

/**
Derive [reified](crate::core::Reify) [values](crate::Value) potential
resolved states.

[Query] is implemented for [`Goals`](crate::goals) and
[`States`](crate::State), meaning you can call
[`.query()`](Query::query()) on them with any type that implements
[`Reify`].

This is a convenient wrapper around the common pattern of obtaining a [`StateIter`](crate::core::StateIter), calling
[`.reify(query)`](crate::core::State::reify()) on each state and returning only the
valid, fully resolved results. Query is implemented on a variety of
[`State`](crate::State) related types, allowing it to be used in many
contexts.

A blanket impl covers anything that implements [`StateIterator`], so many
types including [`Goal`](crate::goals) and [`State`](crate::State) are
queryable.
*/
pub trait Query<'a> {
    /**
    Get [reified](crate::core::Reify) results from things that can produce [`StateIter`](crate::core::StateIter)s.

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
    fn query<Q: Reify + 'a>(self, query: Q) -> Box<dyn Iterator<Item = Q::Reified> + 'a>;
}

impl<'a, S: StateIterator + 'a> Query<'a> for S {
    fn query<Q: Reify + 'a>(self, query: Q) -> Box<dyn Iterator<Item = Q::Reified> + 'a> {
        Box::new(
            self.into_states()
                .filter_map(move |resolved| query.reify_in(&resolved)),
        )
    }
}
