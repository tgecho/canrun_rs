use super::mkmvmap::MKMVMap;

use super::constraints::Constraint;
use crate::{
    core::{AnyVal, Fork, Unify, Value, VarId},
    Reify,
};
use std::rc::Rc;

/** The core struct used to contain and manage [`Value`] bindings.

An open [State] can be updated in a few different ways. Most update methods
return an `Option<State>` to reflect the fact each new constraint can
invalidate the state. This gives you the ability to quickly short circuit with the
[`?` operator](https://doc.rust-lang.org/reference/expressions/operator-expr.html#the-question-mark-operator)
as soon the state hits a dead end.

A [`State`] is designed to be cheap to `clone()`, so make a copy if you want
to try multiple paths.

In general, it is most ergonomic to manipulate a state inside a function
that returns an `Option<State>` to allow the use of the question mark
operator (Note that the [`.apply()`](State::apply()) function makes it easy
to do this).

```
use canrun::{State, Value};

fn my_fn() -> Option<State> {
    let x = Value::var();
    let y = Value::var();
    let state = State::new();
    let maybe: Option<State> = state.unify(&x, &Value::new(1));
    maybe?.unify(&x, &y)
}
assert!(my_fn().is_some());
```
*/
#[derive(Clone)]
pub struct State {
    pub(crate) values: im_rc::HashMap<VarId, AnyVal>,
    pub(crate) forks: im_rc::Vector<Rc<dyn Fork>>,
    constraints: MKMVMap<VarId, Rc<dyn Constraint>>,
}

impl State {
    /**
    Create a new, empty state.

    This often does not need to be used directly as you can
    [`.query()`](crate::Query::query()) a [`Goal`](crate::goals::Goal)
    directly, which handles the state creation internally.

    However, there are use cases for creating and managing a state
    independently of any goals.

    # Example:
    ```
    use canrun::{State};
    let state = State::new();
    ```
    */
    pub fn new() -> Self {
        State {
            values: im_rc::HashMap::new(),
            forks: im_rc::Vector::new(),
            constraints: MKMVMap::new(),
        }
    }

    /**
    Apply an arbitrary function to a state.

    This is primarily a helper to make it easier to get into a function
    where you can use the question mark operator while applying multiple
    updates to a state.

    # Example:
    ```
    use canrun::{State, Query, Value};

    let state = State::new();
    let x = Value::var();
    let state = state.apply(|s| {
        s.unify(&x, &Value::new(1))?
         .unify(&Value::new(1), &x)
    });
    let results: Vec<_> = state.query(x).collect();
    assert_eq!(results, vec![1]);
    ```
    */
    pub fn apply<F>(self, func: F) -> Option<Self>
    where
        F: Fn(Self) -> Option<Self>,
    {
        func(self)
    }

    fn resolve_any<'a>(&'a self, val: &'a AnyVal) -> &'a AnyVal {
        match val {
            AnyVal::Var(var) => {
                let resolved = self.values.get(var);
                match resolved {
                    Some(AnyVal::Var(found_var)) if found_var == var => val,
                    Some(found) => self.resolve_any(found),
                    None => val,
                }
            }
            value => value,
        }
    }

    /** Recursively resolve a [`Value`] as far as the currently
    known variable bindings allow.

    This will return either the final [`Value::Resolved`] (if found) or the
    last [`Value::Var`] it attempted to resolve. It will not force
    [`forks`](State::fork()) to enumerate all potential states, so potential
    bindings that may eventually become confirmed are not considered. Use
    [`StateIterator::into_states`](super::state_iterator::StateIterator::into_states)
    if you want to attempt resolving against all (known) possible states.

    # Example:
    ```
    use canrun::{State, Query, Value};

    # fn test() -> Option<()> {
    let state = State::new();

    let x = Value::var();
    assert_eq!(state.resolve(&x), x);

    let state = state.unify(&x, &Value::new(1))?;
    assert_eq!(state.resolve(&x), Value::new(1));
    # Some(())
    # }
    # test();
    ```
    */
    pub fn resolve<T: Unify>(&self, val: &Value<T>) -> Value<T> {
        self.resolve_any(&val.to_anyval())
            .to_value()
            // I think this should be safe, so long as we are careful to only
            // store a var with the correct type internally.
            .expect("AnyVal resolved to unexpected Value<T>")
    }

    /**
    Attempt to [unify](crate::Unify) two values with each other.

    If the unification fails, [`None`](std::option::Option::None) will be
    returned. [`Value::Var`]s will be checked against relevant
    [constraints](State::constrain), which can also cause a state to fail.

    # Examples:

    ```
    use canrun::{State, Query, Value};

    let x = Value::var();

    let state = State::new();
    let state = state.unify(&x, &Value::new(1));
    assert!(state.is_some());
    ```

    ```
    # use canrun::{State, Query, Value};
    let state = State::new();
    let state = state.unify(&Value::new(1), &Value::new(2));
    assert!(state.is_none());
    ```
    */
    pub fn unify<T: Unify>(mut self, a: &Value<T>, b: &Value<T>) -> Option<Self> {
        let a = self.resolve(a);
        let b = self.resolve(b);

        match (a, b) {
            (Value::Resolved(a), Value::Resolved(b)) => Unify::unify(self, a, b),
            (Value::Var(a), Value::Var(b)) if a == b => Some(self),
            (Value::Var(key), value) | (value, Value::Var(key)) => {
                // TODO: Add occurs check?
                self.values.insert(key.id, value.to_anyval());

                // check constraints matching newly assigned lvar
                if let Some(constraints) = self.constraints.extract(&key.id) {
                    constraints
                        .into_iter()
                        .try_fold(self, |state, func| state.constrain(func))
                } else {
                    Some(self)
                }
            }
        }
    }

    /**
    Add a constraint to the store that can be reevaluated as variables are resolved.

    Some logic is not easy or even possible to express until the resolved
    values are available. `.constrain()` provides a low level way to run
    custom imperative code whenever certain bindings are updated.

    See the [`Constraint` trait](crate::core::constraints::Constraint) for more usage information.
    */
    pub fn constrain(mut self, constraint: Rc<dyn Constraint>) -> Option<Self> {
        match constraint.attempt(&self) {
            Ok(resolve) => resolve(self),
            Err(watch) => {
                self.constraints.add(watch.0, constraint);
                Some(self)
            }
        }
    }

    /**
    Add a potential fork point to the state.

    If there are many possibilities for a certain value or set of values,
    this method allows you to add a [`Fork`] object that can enumerate those
    possible alternate states.

    While this is not quite as finicky as
    [`Constraints`](State::constrain()), you still probably want to use the
    [`any`](crate::goals::any!) or [`either`](crate::goals::either()) goals.

    [Unification](State::unify()) is performed eagerly as soon as it is
    called. [Constraints](State::constrain()) are run as variables are
    resolved. Forking is executed lazily at the end, when
    [`StateIterator::into_states`](super::state_iterator::StateIterator::into_states)
    or [`.query()`](crate::Query::query()) is called.
    */
    pub fn fork(mut self, fork: impl Fork) -> Option<Self> {
        self.forks.push_back(Rc::new(fork));
        Some(self)
    }

    /** Attempt to [reify](crate::core::Reify) the value of a [logic
    variable](crate::core::LVar) in a state.

    # Example:
    ```
    use canrun::{State, StateIterator, Value, LVar};

    let x = LVar::new();

    let state = State::new()
        .unify(&x.into(), &Value::new(1));

    let results: Vec<_> = state.into_states()
        .map(|resolved| resolved.reify(x))
        .collect();

    assert_eq!(results, vec![Some(1)]);
    ```
    */
    pub fn reify<T, R>(&self, value: T) -> Option<R>
    where
        T: Reify<Reified = R>,
    {
        value.reify_in(self)
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use crate::core::*;

    use super::*;

    #[test]
    fn basic_unify() {
        let x = Value::var();
        let state = State::new();
        let state = state.unify(&x, &Value::new(1)).unwrap();
        assert_eq!(state.resolve(&x), Value::new(1));
    }

    #[test]
    fn basic_fork() {
        let x = LVar::new();
        let state: State = State::new();
        let results = state
            .fork(move |s: &State| -> StateIter {
                let s1 = s.clone().unify(&x.into(), &Value::new(1));
                let s2 = s.clone().unify(&x.into(), &Value::new(2));
                Box::new(s1.into_iter().chain(s2.into_iter()))
            })
            .into_states()
            .map(|s| s.resolve(&x.into()))
            .collect::<Vec<_>>();
        assert_eq!(results, vec![Value::new(1), Value::new(2)]);
    }
}
