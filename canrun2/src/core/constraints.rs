//! Run code when [`variables`](LVar) are resolved.

use std::{fmt::Debug, rc::Rc};

use super::{State, Unify};
use crate::core::{LVar, Value, Value::*, VarId};

/**
An alias for the function that should be returned by a successful
[`Constraint::attempt`] to update the [`State`].
*/
pub type ResolveFn = Box<dyn FnOnce(State) -> Option<State>>;

/**
A set of variables to watch on behalf of a [`Constraint`].

Consider generating this with the [`resolve_1`], [`resolve_2`], [`OneOfTwo`]
or [`TwoOfThree`] helpers.
*/
#[derive(Debug)]
pub struct VarWatch(pub(crate) Vec<VarId>);

impl VarWatch {
    /// Watch one [`LVar`] for changes in a [`Constraint`].
    pub fn one<A>(a: LVar<A>) -> Self {
        VarWatch(vec![a.id])
    }

    /// Watch two [`LVar`]s for changes in a [`Constraint`].
    pub fn two<A, B>(a: LVar<A>, b: LVar<B>) -> Self {
        VarWatch(vec![a.id, b.id])
    }
}

/** Update a [`State`] whenever one or more [`LVar`]s are resolved.

The [`Constraint::attempt`] function will be run when it is initially added.
Returning a [`Err([VarWatch])`](VarWatch) signals that the constraint is not
satisfied. The constraint will be re-attempted when one of the specified
variables is bound to another value.

You probably want to use the higher level [goal projection](crate::goals::project)
functions.

# NOTE:
The [`attempt`](Constraint::attempt) function must take care to [fully
resolve](State::resolve) any variables before creating a [`VarWatch`].
The [`resolve_1`], [`resolve_2`], [`OneOfTwo`] and [`TwoOfThree`]
helpers can simplify handling this (plus returning the [`VarWatch`]).

# Example:
```
use canrun2::{State, Unify, Query, Value};
use canrun2::constraints::{Constraint, resolve_1, ResolveFn, VarWatch};
use std::rc::Rc;

struct Assert<T: Unify> {
    val: Value<T>,
    assert: Rc<dyn Fn(&T) -> bool>,
}

impl<T: Unify> Constraint for Assert<T>
{
    fn attempt(&self, state: &State) -> Result<ResolveFn, VarWatch> {
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
    fn attempt(&self, state: &State) -> Result<ResolveFn, VarWatch>;
}

/// Resolve one [`Value`] or return an [`Err(VarWatch)`](VarWatch) in a
/// [`Constraint`].
pub fn resolve_1<A: Unify>(val: &Value<A>, state: &State) -> Result<Rc<A>, VarWatch> {
    match state.resolve(val) {
        Resolved(a) => Ok(a),
        Var(var) => Err(VarWatch::one(var)),
    }
}

/// Resolve two [`Value`]s or return an [`Err(VarWatch)`](VarWatch) in a
/// [`Constraint`].
pub fn resolve_2<A: Unify, B: Unify>(
    a: &Value<A>,
    b: &Value<B>,
    state: &State,
) -> Result<(Rc<A>, Rc<B>), VarWatch> {
    let a = state.resolve(a);
    let b = state.resolve(b);
    match (a, b) {
        (Resolved(a), Resolved(b)) => Ok((a, b)),
        (Var(var), _) => Err(VarWatch::one(var)),
        (_, Var(var)) => Err(VarWatch::one(var)),
    }
}

/// Resolve one out of two [`Value`]s or return an [`Err(VarWatch)`](VarWatch) in
/// a [`Constraint`].
pub enum OneOfTwo<A: Unify, B: Unify> {
    /// Returned when the first [`Value`] is successfully resolved.
    A(Rc<A>, Value<B>),
    /// Returned when the second [`Value`] is successfully resolved.
    B(Value<A>, Rc<B>),
}

impl<A: Unify, B: Unify> OneOfTwo<A, B> {
    /// Attempt to resolve a [`OneOfTwo`] enum from a [`State`].
    pub fn resolve(a: &Value<A>, b: &Value<B>, state: &State) -> Result<OneOfTwo<A, B>, VarWatch> {
        let a = state.resolve(a);
        let b = state.resolve(b);
        match (a, b) {
            (Resolved(a), b) => Ok(OneOfTwo::A(a, b)),
            (a, Resolved(b)) => Ok(OneOfTwo::B(a, b)),
            (Var(a), Var(b)) => Err(VarWatch::two(a, b)),
        }
    }
}

/// Resolve two out of three [`Value`]s or return an [`Err(VarWatch)`](VarWatch)
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
    /// Attempt to resolve a [`TwoOfThree`] enum from a [`State`].
    pub fn resolve(
        a: &Value<A>,
        b: &Value<B>,
        c: &Value<C>,
        state: &State,
    ) -> Result<TwoOfThree<A, B, C>, VarWatch> {
        let a = state.resolve(a);
        let b = state.resolve(b);
        let c = state.resolve(c);
        match (a, b, c) {
            (Resolved(a), Resolved(b), c) => Ok(TwoOfThree::AB(a, b, c)),
            (a, Resolved(b), Resolved(c)) => Ok(TwoOfThree::BC(a, b, c)),
            (Resolved(a), b, Resolved(c)) => Ok(TwoOfThree::AC(a, b, c)),
            (Var(a), Var(b), _) => Err(VarWatch::two(a, b)),
            (Var(a), _, Var(c)) => Err(VarWatch::two(a, c)),
            (_, Var(b), Var(c)) => Err(VarWatch::two(b, c)),
        }
    }
}
