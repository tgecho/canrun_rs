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
