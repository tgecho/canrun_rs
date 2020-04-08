//! Track value bindings and constraints during the evaluation process.
//!
//! State is the imperative core of each logic program. It manages the updates
//! to the relationships between values while delegating the actual storage to a
//! type specific [Domain](crate::domains).
//!
//! In general, it is preferred to deal with State indirectly through
//! [goals](crate::goal). They are essentially equivalent in capability, and
//! their declarative, higher level nature makes them much easier to use.
//! Notably, goal functions provide automatic [value](crate::value) wrapping
//! through [IntoVal](crate::value::IntoVal).
//!
//! An open [State] is the initial struct that you will start with (explicitly
//! or implicitly through a [goal](crate::goal)). Iterating through the
//! potentially results will yield zero or more [ResolvedStates](ResolvedState).
mod constraint;
mod constraints;
mod impls;
mod iter_resolved;
mod resolved;

use super::util::multikeymultivaluemap::MKMVMap;
use crate::domains::{Domain, DomainType};
use crate::unify::Unify;
use crate::value::{
    LVar, LVarId, Val,
    Val::{Resolved, Var},
};
pub use constraints::{Constraint, WatchList};
pub use iter_resolved::{IterResolved, ResolvedIter};
pub use resolved::ResolvedState;
use std::iter::once;
use std::rc::Rc;

pub type StateIter<'s, D> = Box<dyn Iterator<Item = State<'s, D>> + 's>;
type ConstraintFns<'s, D> =
    MKMVMap<LVarId, Rc<dyn Fn(State<'s, D>) -> Constraint<State<'s, D>> + 's>>;
#[doc(hidden)]
pub use im_rc::HashMap;

/// The core struct used to contain and manage [value](crate::value) bindings.
///
/// An open [State] can be updated in a few different ways. Most update methods
/// return an `Option<State<D>>` to reflect the fact each new constraint
/// invalidate the state. This gives you the ability to quickly short circuit as
/// soon the state hits a dead end.
///
/// In general, it is most ergonomic to manipulate a state inside a function
/// that returns an `Option<State<D>>` to allow the use of the question mark
/// operator (Note that the [.apply()](State::apply()) function makes it easy to
/// do this).
///
/// ```
/// use canrun::{State, val, var};
/// use canrun::domains::example::I32;
///
/// fn my_fn<'a>() -> Option<State<'a, I32>> {
///     let x = var();
///     let y = var();
///     let state: State<I32> = State::new();
///     let maybe: Option<State<I32>> = state.unify(&val!(x), &val!(1));
///     maybe?.unify(&val!(x), &val!(y))
/// }
/// assert!(my_fn().is_some());
/// ```
#[derive(Clone)]
pub struct State<'a, D: Domain<'a> + 'a> {
    domain: D,
    constraints: ConstraintFns<'a, D>,
    forks: im_rc::Vector<Rc<dyn Fn(Self) -> StateIter<'a, D> + 'a>>,
}

impl<'a, D: Domain<'a> + 'a> State<'a, D> {
    pub fn new() -> Self {
        State {
            domain: D::new(),
            constraints: MKMVMap::new(),
            forks: im_rc::Vector::new(),
        }
    }

    pub fn apply<F>(self, func: F) -> Option<Self>
    where
        F: Fn(Self) -> Option<Self>,
    {
        func(self)
    }

    fn iter_forks(mut self) -> StateIter<'a, D> {
        let fork = self.forks.pop_front();
        match fork {
            None => Box::new(once(self)),
            Some(fork) => Box::new(fork(self).flat_map(State::iter_forks)),
        }
    }

    pub fn resolve_val<'r, T>(&'r self, val: &'r Val<T>) -> &'r Val<T>
    where
        D: DomainType<'a, T>,
    {
        match val {
            Val::Var(var) => self.domain.values_as_ref().get(var).unwrap_or(val),
            value => value,
        }
    }

    pub fn get<'g, T>(&'g self, var: LVar<T>) -> Result<&'g T, LVar<T>>
    where
        D: DomainType<'a, T>,
    {
        match self.domain.values_as_ref().get(&var) {
            Some(val) => val.resolved(),
            None => Err(var),
        }
    }

    pub fn unify<T>(mut self, a: &Val<T>, b: &Val<T>) -> Option<Self>
    where
        D: Unify<'a, T>,
    {
        let a = self.resolve_val(a);
        let b = self.resolve_val(b);
        match (a, b) {
            (Resolved(a), Resolved(b)) => {
                let a = a.clone();
                let b = b.clone();
                Unify::unify_resolved(self, a, b)
            }
            (Var(a), Var(b)) if a == b => Some(self),
            (Var(var), val) | (val, Var(var)) => {
                let key = *var;
                let value = val.clone();

                // TODO: Add occurs check?

                // Assign lvar to value
                self.domain.values_as_mut().insert(key, value);

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

    pub fn constrain(self, func: Rc<dyn Fn(Self) -> Constraint<Self> + 'a>) -> Option<Self> {
        match func(self) {
            Constraint::Done(state) => state,
            Constraint::Waiting(mut state, WatchList(vars)) => {
                state.constraints.add(vars, func);
                Some(state)
            }
        }
    }

    pub fn fork(mut self, func: Rc<dyn Fn(Self) -> StateIter<'a, D> + 'a>) -> Option<Self> {
        self.forks.push_back(func);
        Some(self)
    }
}
