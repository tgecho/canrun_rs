//! Make declarative assertions about the relationships between values.
//!
//! [`Goals`](crate::Goal) provide a high level interface for defining logic programs.
//! They are composable, with many higher level goals being made
//! up of lower level primitives. Since the typical way of using goals are
//! through simple functions, it is easy to build and reuse custom, first class
//! goal constructors.
//!
//! While [`State`] exposes a lower level API, in practice there really shouldn't
//! be anything that can't be expressed using goals.
use crate::domains::Domain;
use crate::query::Query;
use crate::state::{Fork, State};
use crate::state::{IterResolved, ResolvedStateIter};
use crate::ReifyIn;
use std::rc::Rc;

mod all;
mod any;
mod both;
mod custom;
mod either;
mod lazy;
pub mod project;
mod unify;

#[doc(inline)]
pub use all::all;
#[doc(inline)]
pub use any::any;
#[doc(inline)]
pub use both::both;
#[doc(inline)]
pub use custom::custom;
#[doc(inline)]
pub use either::either;
#[doc(inline)]
pub use lazy::lazy;
#[doc(inline)]
pub use unify::unify;

#[derive(Clone, Debug)]
pub(crate) enum GoalEnum<'a, D: Domain<'a>> {
    Succeed,
    Fail,
    UnifyIn(D::Value, D::Value),
    Fork(Rc<dyn Fork<'a, D> + 'a>),
    Both(Box<GoalEnum<'a, D>>, Box<GoalEnum<'a, D>>),
    All(Vec<GoalEnum<'a, D>>),
    Lazy(lazy::Lazy<'a, D>),
    Custom(custom::Custom<'a, D>),
    Project(Rc<dyn project::Project<'a, D> + 'a>),
}

/// A container of one of many possible types of [goals](crate::goal).
///
/// Values of this type are typically constructed with one of the many
/// [constructor functions](crate::goal#functions) and
/// [macros](crate::goal#macros). These high level methods provide automatic
/// [value](crate::value) wrapping through [`IntoVal`](crate::value::IntoVal) and
/// other niceties.
#[derive(Clone, Debug)]
pub struct Goal<'a, D: Domain<'a>>(GoalEnum<'a, D>);

impl<'a, D: Domain<'a> + 'a> GoalEnum<'a, D> {
    fn apply(self, state: State<'a, D>) -> Option<State<'a, D>> {
        match self {
            GoalEnum::Succeed => Some(state),
            GoalEnum::Fail => None,
            GoalEnum::UnifyIn(a, b) => unify::run(state, a, b),
            GoalEnum::Fork(fork) => state.fork(fork),
            GoalEnum::Both(a, b) => both::run(state, *a, *b),
            GoalEnum::All(goals) => all::run(state, goals),
            GoalEnum::Lazy(lazy) => lazy.run(state),
            GoalEnum::Custom(custom) => custom.run(state),
            GoalEnum::Project(proj) => project::run(proj, state),
        }
    }
}

impl<'a, D: Domain<'a> + 'a> Goal<'a, D> {
    /// Create a Goal that always succeeds.
    ///
    /// # Example
    /// ```
    /// use canrun::{Goal, all, unify, var};
    /// use canrun::domains::example::I32;
    ///
    /// let x = var();
    /// let goal: Goal<I32> = all![unify(x, 1), Goal::succeed()];
    /// let result: Vec<_> = goal.query(x).collect();
    /// assert_eq!(result, vec![1])
    /// ```
    pub fn succeed() -> Self {
        Goal(GoalEnum::Succeed)
    }

    /// Create a Goal that always fails.
    ///
    /// # Example
    /// ```
    /// use canrun::{Goal, all, unify, var};
    /// use canrun::domains::example::I32;
    ///
    /// let x = var();
    /// let goal: Goal<I32> = all![unify(x, 1), Goal::fail()];
    /// let result: Vec<_> = goal.query(x).collect();
    /// assert_eq!(result, vec![])
    /// ```
    pub fn fail() -> Self {
        Goal(GoalEnum::Fail)
    }

    /// Create a goal containing a [`Fork` object](crate::state::Fork).
    pub fn fork<F: Fork<'a, D> + 'a>(fork: F) -> Self {
        Goal(GoalEnum::Fork(Rc::new(fork)))
    }

    /// Create a Goal that only succeeds if all sub-goals succeed.
    ///
    /// This constructor takes anything that implements
    /// [`IntoIterator`](std::iter::IntoIterator) for a compatible goal type.
    /// See the [`all!`](./macro.all.html) macro for a slightly higher level
    /// interface.
    ///
    /// # Example
    /// ```
    /// use canrun::{Goal, all, unify, var};
    /// use canrun::domains::example::I32;
    ///
    /// let x = var();
    /// let y = var();
    /// let goal: Goal<I32> = Goal::all(vec![unify(y, x), unify(1, x), unify(y, 1)]);
    /// let result: Vec<_> = goal.query((x, y)).collect();
    /// assert_eq!(result, vec![(1, 1)])
    /// ```
    pub fn all<I: IntoIterator<Item = Goal<'a, D>>>(goals: I) -> Self {
        Goal(GoalEnum::All(goals.into_iter().map(|g| g.0).collect()))
    }

    /// Create a Goal that yields a state for every successful
    /// sub-goal.
    ///
    /// This constructor takes anything that implements
    /// [`IntoIterator`](std::iter::IntoIterator) for a compatible goal type. See the
    /// [`any!`](./macro.any.html) macro for a slightly higher level interface.
    ///
    /// # Example
    /// ```
    /// use canrun::{Goal, any, unify, var};
    /// use canrun::domains::example::I32;
    ///
    /// let x = var();
    /// let goal: Goal<I32> = Goal::any(vec![unify(x, 1), unify(x, 2), unify(x, 3)]);
    /// let result: Vec<_> = goal.query(x).collect();
    /// assert_eq!(result, vec![1, 2, 3])
    /// ```
    pub fn any<I: IntoIterator<Item = Goal<'a, D>>>(goals: I) -> Self {
        Goal::fork(any::Any {
            goals: goals.into_iter().map(|g| g.0).collect(),
        })
    }

    /// Create a Goal that deals with resolved values.
    ///
    /// See the [`Project`](crate::goal::project::Project) trait for details.
    pub fn project<P: project::Project<'a, D> + 'a>(proj: P) -> Self {
        Goal(GoalEnum::Project(Rc::new(proj)))
    }

    /// Apply the Goal to an existing state.
    ///
    /// This will update the state, but not iterate through the possible
    /// resolved states. For this you still need to use the
    /// [`.iter_resolved()`](IterResolved::iter_resolved()) interface or
    /// [`.query()`](Goal::query()).
    ///
    /// # Example
    /// ```
    /// use canrun::{Goal, State, unify, var};
    /// use canrun::domains::example::I32;
    ///
    /// let x = var();
    /// let state = State::new();
    /// let goal: Goal<I32> = unify(x, 1);
    /// let state: Option<State<I32>> = goal.apply(state);
    /// ```
    pub fn apply(self, state: State<'a, D>) -> Option<State<'a, D>> {
        self.0.apply(state)
    }

    /// Use the [query](crate::query) interface to get an iterator of result
    /// values.
    ///
    /// This is a shorthand for creating a new state, applying the goal and
    /// calling [`.query()`](crate::Query) on the resulting state.
    ///
    /// # Example:
    /// ```
    /// use canrun::{Goal, unify, var};
    /// use canrun::domains::example::I32;
    ///
    /// let x = var();
    /// let goal: Goal<I32> = unify(x, 1);
    /// let result: Vec<_> = goal.query(x).collect();
    /// assert_eq!(result, vec![1])
    /// ```
    pub fn query<Q>(self, query: Q) -> Box<dyn Iterator<Item = Q::Reified> + 'a>
    where
        Q: ReifyIn<'a, D> + 'a,
    {
        Query::query(self, query)
    }
}

impl<'a, D: Domain<'a> + 'a> IterResolved<'a, D> for Goal<'a, D> {
    fn iter_resolved(self) -> ResolvedStateIter<'a, D> {
        self.apply(State::new()).iter_resolved()
    }
}
