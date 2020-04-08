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

mod constraints;
mod impls;
mod iter_resolved;
mod resolved;

use super::util::multikeymultivaluemap::MKMVMap;
use crate::domains::{Domain, DomainType};
use crate::unify::Unify;
use crate::value::{
    LVarId, Val,
    Val::{Resolved, Var},
};
pub use constraints::{Constraint, WatchList};
pub use iter_resolved::{IterResolved, ResolvedStateIter};
pub use resolved::ResolvedState;
use std::iter::once;
use std::rc::Rc;

pub(crate) type StateIter<'s, D> = Box<dyn Iterator<Item = State<'s, D>> + 's>;
type ConstraintFns<'s, D> =
    MKMVMap<LVarId, Rc<dyn Fn(State<'s, D>) -> Constraint<State<'s, D>> + 's>>;

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
    /// Create a new, empty state.
    ///
    /// This often does not need to be used directly as you can
    /// [`.query()`](crate::goal::Goal::query()) a [`Goal`](crate::goal::Goal)
    /// directly, which handles the state creation internally.
    ///
    /// However, there are use cases for creating and managing a state
    /// independently of any goals.
    ///
    /// # Example:
    /// ```
    /// use canrun::{State, var};
    /// use canrun::domains::example::I32;
    ///
    /// let state: State<I32> = State::new();
    /// ```
    pub fn new() -> Self {
        State {
            domain: D::new(),
            constraints: MKMVMap::new(),
            forks: im_rc::Vector::new(),
        }
    }

    /// Apply an arbitrary function to a state.
    ///
    /// This is primarily a helper to make it easier to get into a function where
    /// you can use the question mark operator while applying multiple updates
    /// to a state.
    ///
    /// # Example:
    /// ```
    /// use canrun::{State, Queryable, val, var};
    /// use canrun::domains::example::I32;
    ///
    /// let s: State<I32> = State::new();
    /// let x = var();
    /// let s = s.apply(|s| {
    ///     s.unify(&val!(x), &val!(1))?
    ///      .unify(&val!(1), &val!(x))
    /// });
    /// let results: Vec<i32> = s.query(x).collect();
    /// assert_eq!(results, vec![1]);
    /// ```
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

    /// Recursively resolve a [Val](crate::value::Val) as far as the currently
    /// known variable bindings allow.
    ///
    /// This will return either the final [`Val::Resolved`] (if found) or the
    /// last [`Val::Var`] it attempted to resolve. It will not force
    /// [`forks`](State::fork()) to enumerate, so potential bindings are not
    /// considered.
    ///
    /// # Example:
    /// ```
    /// use canrun::{State, Queryable, val, var};
    /// use canrun::domains::example::I32;
    ///
    /// # fn test() -> Option<()> {
    /// let state: State<I32> = State::new();
    ///
    /// let x = val!(var());
    /// assert_eq!(state.resolve_val(&x), &x);
    ///
    /// let state = state.unify(&x, &val!(1))?;
    /// assert_eq!(state.resolve_val(&x), &val!(1));
    /// # Some(())
    /// # }
    /// # test();
    /// ```
    pub fn resolve_val<'r, T>(&'r self, val: &'r Val<T>) -> &'r Val<T>
    where
        D: DomainType<'a, T>,
    {
        match val {
            Val::Var(var) => self.domain.values_as_ref().0.get(var).unwrap_or(val),
            value => value,
        }
    }

    /// Attempt to [unify](crate::unify) two values with each other.
    ///
    /// If the unification fails, [`None`](std::option::Option::None) will be
    /// returned. [Val::Var]s will be checked against relevant
    /// [constraints](State::constrain), which can also cause a state to fail.
    ///
    ///  # Examples:
    /// ```
    /// use canrun::{State, Queryable, val, var};
    /// use canrun::domains::example::I32;
    ///
    /// let x = val!(var());
    ///
    /// let state: State<I32> = State::new();
    /// let state = state.unify(&x, &val!(1));
    /// assert!(state.is_some());
    /// ```
    /// ```
    /// # use canrun::{State, Queryable, val};
    /// # use canrun::domains::example::I32;
    /// let state: State<I32> = State::new();
    /// let state = state.unify(&val!(1), &val!(2));
    /// assert!(state.is_none());
    /// ```
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
                self.domain.values_as_mut().0.insert(key, value);

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

    /// Add a constraint to the store that can be reevaluated as variables are
    /// resolved.
    ///
    /// Some logic is not easy or even possible to express until the resolved
    /// values are available. `.constrain()` provides a low level way to run
    /// custom imperative code whenever certain bindings are updated.
    ///
    /// The constraint function will be run when it is initially added.
    /// Returning a [Waiting](Constraint::Waiting) value signals that the
    /// constraint is not satisfied. It will be re-run whenever one of the
    /// specified variables is bound to another value.
    ///
    /// *This is a pretty raw interface.* You should probably use the higher level
    /// [goal projection](crate::goal::project) functionality.
    ///
    /// # Invariants:
    /// If the constraint is able to fully resolve enough values to complete, it
    /// may update the state to reflect additional assignments or constraints.
    /// It must only update the state IF it is satisfied.
    ///
    /// Additionally, the constraint function must take care to [fully
    /// resolve](State::resolve_val) any variables before
    /// [Waiting](Constraint::Waiting) on them.
    ///
    /// This is admittedly more than a bit rough.
    ///
    /// # Example:
    /// ```
    /// use canrun::{State, Queryable, val, var};
    /// use canrun::state::Constraint;
    /// use canrun::domains::example::I32;
    /// use std::rc::Rc;
    ///
    /// # fn test() -> Option<()> {
    /// let x = var();
    ///
    /// let state: State<I32> = State::new();
    /// let state = state.constrain(Rc::new(move |s| {
    ///     match s.resolve_val(&val!(x)).resolved() {
    ///         Ok(resolved) => Constraint::Done(
    ///             if *resolved > 0 {
    ///                 Some(s)
    ///             } else {
    ///                 None
    ///             }
    ///         ),
    ///         Err(unresolved) => Constraint::on_1(s, unresolved),
    ///     }
    /// }));
    /// let state = state?.unify(&val!(x), &val!(1));
    ///
    /// let results: Vec<i32> = state.query(x).collect();
    /// assert_eq!(results, vec![1]);
    /// # Some(())
    /// # }
    /// # test();
    /// ```
    pub fn constrain(self, func: Rc<dyn Fn(Self) -> Constraint<Self> + 'a>) -> Option<Self> {
        match func(self) {
            Constraint::Done(state) => state,
            Constraint::Waiting(mut state, WatchList(vars)) => {
                state.constraints.add(vars, func);
                Some(state)
            }
        }
    }

    /// Add a potential fork function to the state.
    ///
    /// If there are many possibilities for a certain value or set of values,
    /// this method allows you to add a function that can enumerate those
    /// possible alternate states.
    ///
    /// While this is not quite as finicky as the
    /// [Constraints](State::constrain()), you still probably want to use the
    /// [`any`](crate::goal::any) or [`either`](crate::goal::either) goals.
    ///
    /// [Unification](State::unify()) is performed eagerly as soon as it is
    /// called. [Constraints](State::constrain()) are run as variables are
    /// resolved. Forking is only executed at the end, when
    /// [.iter_resolved()](crate::state::IterResolved::iter_resolved()) (or
    /// [`.query()](crate::query::Query())) is called.
    ///
    ///  # Example:
    /// ```
    /// use canrun::{State, Queryable, val, var};
    /// use canrun::domains::example::I32;
    /// use std::rc::Rc;
    ///
    /// let x = var();
    /// let state: State<I32> = State::new();
    ///
    /// let state = state.fork(Rc::new(|s: State<I32>| {
    ///     let s1: Option<State<I32>> = s.clone().unify(&val!(x), &val!(1));
    ///     let s2: Option<State<I32>> = s.unify(&val!(x), &val!(2));
    ///     Box::new(s1.into_iter().chain(s2.into_iter()))
    /// }));
    /// let results: Vec<i32> = state.query(x).collect();
    /// assert_eq!(results, vec![1, 2]);
    /// ```
    pub fn fork(mut self, func: Rc<dyn Fn(Self) -> StateIter<'a, D> + 'a>) -> Option<Self> {
        self.forks.push_back(func);
        Some(self)
    }
}
