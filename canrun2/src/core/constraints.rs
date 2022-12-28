use std::{fmt::Debug, rc::Rc};

use super::{State, Unify};
use crate::value::{LVar, Value, Value::*, VarId};

pub type ResolveFn = Box<dyn FnOnce(State) -> Option<State>>;

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

pub trait Constraint {
    /// Resolve required variables in a state and resubscribe or request to
    /// update the state.
    fn attempt(&self, state: &State) -> Result<ResolveFn, VarWatch>;
}

/// Resolve one [`Val`] or return an [`Err(VarWatch)`](VarWatch) in a
/// [`Constraint`].
pub fn resolve_1<A: Unify>(val: &Value<A>, state: &State) -> Result<Rc<A>, VarWatch> {
    match state.resolve(val) {
        Resolved(a) => Ok(a),
        Var(var) => Err(VarWatch::one(var)),
    }
}

/// Resolve two [`Val`]s or return an [`Err(VarWatch)`](VarWatch) in a
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

/// Resolve one out of two [`Val`]s or return an [`Err(VarWatch)`](VarWatch) in
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

/// Resolve two out of three [`Val`]s or return an [`Err(VarWatch)`](VarWatch)
/// in a [`Constraint`].
pub enum TwoOfThree<A: Unify, B: Unify, C: Unify> {
    /// Returned when the first and second [`Val`]s are successfully resolved.
    AB(Rc<A>, Rc<B>, Value<C>),
    /// Returned when the second and third [`Val`]s are successfully resolved.
    BC(Value<A>, Rc<B>, Rc<C>),
    /// Returned when the first and third [`Val`]s are successfully resolved.
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
