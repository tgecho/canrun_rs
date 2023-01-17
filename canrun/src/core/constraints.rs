//! Run code when [`variables`](crate::LVar) are resolved.

use std::rc::Rc;

use super::{State, Unify};
use crate::{
    core::{Value, Value::*},
    LVarList,
};

/**
An alias for the function that should be returned by a successful
[`Constraint::attempt`] to update the [`State`].
*/
pub type ResolveFn = Box<dyn FnOnce(State) -> Option<State>>;

/** Update a [`State`] whenever one or more [`crate::LVar`]s are resolved.

The [`Constraint::attempt`] function will be run when it is initially added.
Returning a [`Err([LVarList])`](LVarList) signals that the constraint is not
satisfied. The constraint will be re-attempted when one of the specified
variables is bound to another value.

You probably want to use the higher level [goal projection](crate::goals::project)
functions.

# NOTE:
The [`attempt`](Constraint::attempt) function must take care to [fully
resolve](State::resolve) any variables before creating a [`LVarList`].
The [`resolve_1`], [`resolve_2`], [`OneOfTwo`] and [`TwoOfThree`]
helpers can simplify handling this (plus returning the [`LVarList`]).

# Example:
```
use canrun::{State, Unify, Query, Value, LVarList};
use canrun::constraints::{Constraint, resolve_1, ResolveFn};
use std::rc::Rc;

struct Assert<T: Unify> {
    val: Value<T>,
    assert: Rc<dyn Fn(&T) -> bool>,
}

impl<T: Unify> Constraint for Assert<T>
{
    fn attempt(&self, state: &State) -> Result<ResolveFn, LVarList> {
        let resolved = resolve_1(&self.val, state)?;
        let assert = self.assert.clone();
        Ok(Box::new(
            move |state: State| if assert(&*resolved) { Some(state) } else { None },
        ))
    }
}

# fn test() -> Option<()> {
let x = Value::var();

let state = State::new();
let state = state.constrain(Rc::new(Assert {val: x.clone(), assert: Rc::new(|x| x > &1)}));
let state = state?.unify(&x, &Value::new(2));

let results: Vec<i32> = state.query(x).collect();
assert_eq!(results, vec![2]);
# Some(())
# }
# test();
```
*/
pub trait Constraint {
    /// Resolve required variables in a state and resubscribe or request to
    /// update the state.
    fn attempt(&self, state: &State) -> Result<ResolveFn, LVarList>;
}

/// Resolve one [`Value`] or return an [`Err(LVarList)`](LVarList) in a
/// [`Constraint`].
pub fn resolve_1<A: Unify>(val: &Value<A>, state: &State) -> Result<Rc<A>, LVarList> {
    match state.resolve(val) {
        Resolved(a) => Ok(a),
        Var(var) => Err(LVarList::one(var)),
    }
}

/// Resolve two [`Value`]s or return an [`Err(LVarList)`](LVarList) in a
/// [`Constraint`].
pub fn resolve_2<A: Unify, B: Unify>(
    a: &Value<A>,
    b: &Value<B>,
    state: &State,
) -> Result<(Rc<A>, Rc<B>), LVarList> {
    let a = state.resolve(a);
    let b = state.resolve(b);
    match (a, b) {
        (Resolved(a), Resolved(b)) => Ok((a, b)),
        (Var(var), _) => Err(LVarList::one(var)),
        (_, Var(var)) => Err(LVarList::one(var)),
    }
}

/// Resolve one out of two [`Value`]s or return an [`Err(LVarList)`](LVarList) in
/// a [`Constraint`].
pub enum OneOfTwo<A: Unify, B: Unify> {
    /// Returned when the first [`Value`] is successfully resolved.
    A(Rc<A>, Value<B>),
    /// Returned when the second [`Value`] is successfully resolved.
    B(Value<A>, Rc<B>),
}

impl<A: Unify, B: Unify> OneOfTwo<A, B> {
    /**
    Attempt to resolve a [`OneOfTwo`] enum from a [`State`].

    Examples:
    If neither var is resolved, you'll get back an [`Err<LVarList>`](crate::LVarList)
    suitable for returning from a [`Constraint::attempt()`].
    ```
    use canrun::{State, Value, LVarList};
    use canrun::constraints::OneOfTwo;

    let state = State::new();
    let a: Value<usize> = Value::var();
    let b: Value<usize> = Value::var();
    let resolved = OneOfTwo::resolve(&a, &b, &state);
    assert!(resolved.is_err());
    ```
    If one of the vars is able to be resolved from the [`State`], the return value will
    be an `Ok` containing the unresolved [`Value::Var`] and the resolved value in an `Rc`.
    ```
    # use std::rc::Rc;
    # use canrun::{State, Value, LVarList};
    # use canrun::constraints::OneOfTwo;

    # let state = State::new();
    # let a: Value<usize> = Value::var();
    # let b: Value<usize> = Value::var();
    let state = state.unify(&b, &Value::new(1)).unwrap();
    let resolved = OneOfTwo::resolve(&a, &b, &state);
    match resolved {
        Ok(OneOfTwo::B(var, b)) => assert_eq!(*b, 1),
        # _ => panic!(),
    }
    ```
    */
    pub fn resolve(a: &Value<A>, b: &Value<B>, state: &State) -> Result<OneOfTwo<A, B>, LVarList> {
        let a = state.resolve(a);
        let b = state.resolve(b);
        match (a, b) {
            (Resolved(a), b) => Ok(OneOfTwo::A(a, b)),
            (a, Resolved(b)) => Ok(OneOfTwo::B(a, b)),
            (Var(a), Var(b)) => Err(LVarList::two(a, b)),
        }
    }
}

/// Resolve two out of three [`Value`]s or return an [`Err(LVarList)`](LVarList)
/// in a [`Constraint`].
pub enum TwoOfThree<A: Unify, B: Unify, C: Unify> {
    /// Returned when the first and second [`Value`]s are successfully resolved.
    AB(Rc<A>, Rc<B>, Value<C>),
    /// Returned when the second and third [`Value`]s are successfully resolved.
    BC(Value<A>, Rc<B>, Rc<C>),
    /// Returned when the first and third [`Value`]s are successfully resolved.
    AC(Rc<A>, Value<B>, Rc<C>),
}

impl<A: Unify, B: Unify, C: Unify> TwoOfThree<A, B, C> {
    /**
    Attempt to resolve a [`TwoOfThree`] enum from a [`State`].

    Examples:
    If no vars are resolved, you'll get back an [`Err<LVarList>`](crate::LVarList)
    suitable for returning from a [`Constraint::attempt()`].
    ```
    use canrun::{State, Value, LVarList};
    use canrun::constraints::TwoOfThree;

    let state = State::new();
    let a: Value<usize> = Value::var();
    let b: Value<usize> = Value::var();
    let c: Value<usize> = Value::var();
    let resolved = TwoOfThree::resolve(&a, &b, &c, &state);
    assert!(resolved.is_err());
    ```
    If two of the vars is able to be resolved from the [`State`], the return value will
    be an `Ok` containing the unresolved [`Value::Var`] and the resolved value in an `Rc`.
    ```
    # use std::rc::Rc;
    # use canrun::{State, Value, LVarList};
    # use canrun::constraints::TwoOfThree;

    # let state = State::new();
    # let a: Value<usize> = Value::var();
    # let b: Value<usize> = Value::var();
    # let c: Value<usize> = Value::var();
    let state = state.unify(&a, &Value::new(1)).unwrap();
    let state = state.unify(&b, &Value::new(2)).unwrap();
    let resolved = TwoOfThree::resolve(&a, &b, &c, &state);
    match resolved {
        Ok(TwoOfThree::AB(a, b, var)) => {
            assert_eq!(*a, 1);
            assert_eq!(*b, 2);
        },
        # _ => panic!(),
    }
    ```
    */
    pub fn resolve(
        a: &Value<A>,
        b: &Value<B>,
        c: &Value<C>,
        state: &State,
    ) -> Result<TwoOfThree<A, B, C>, LVarList> {
        let a = state.resolve(a);
        let b = state.resolve(b);
        let c = state.resolve(c);
        match (a, b, c) {
            (Resolved(a), Resolved(b), c) => Ok(TwoOfThree::AB(a, b, c)),
            (a, Resolved(b), Resolved(c)) => Ok(TwoOfThree::BC(a, b, c)),
            (Resolved(a), b, Resolved(c)) => Ok(TwoOfThree::AC(a, b, c)),
            (Var(a), Var(b), _) => Err(LVarList::two(a, b)),
            (Var(a), _, Var(c)) => Err(LVarList::two(a, c)),
            (_, Var(b), Var(c)) => Err(LVarList::two(b, c)),
        }
    }
}
