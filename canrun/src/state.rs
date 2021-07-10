/*! Track value bindings and constraints during the evaluation process.

State is the imperative core of each logic program. It manages the updates
to the relationships between values while delegating the actual storage to a
type specific [`Domain`](crate::domains).

In general, it is preferred to deal with State indirectly through
[goals](crate::goals). They are essentially equivalent in capability, and
their declarative, higher level nature makes them much easier to use.
Goal functions typically provide automatic [value](crate::value) wrapping
through [`IntoVal`](crate::value::IntoVal).

An open [State] is the initial struct that you will start with (explicitly
or implicitly through a [goal](crate::goals)). Iterating through the
potential results will yield zero or more
[`ResolvedStates`](ResolvedState).
*/

pub mod constraints;
mod impls;
mod iter_resolved;
mod resolved;

use super::util::multikeymultivaluemap::MKMVMap;
use crate::domains::{Domain, DomainType};
use crate::value::{
    LVarId, Val,
    Val::{Resolved, Var},
};
use crate::UnifyIn;
#[doc(hidden)]
pub use constraints::Constraint;
pub use iter_resolved::{IterResolved, ResolvedStateIter};
pub use resolved::ResolvedState;
use std::fmt::Debug;
use std::iter::once;
use std::rc::Rc;

/// Type alias for an [`Iterator`] of [`States`](crate::state::State)
pub type StateIter<'s, D> = Box<dyn Iterator<Item = State<'s, D>> + 's>;
type ConstraintFns<'s, D> = MKMVMap<LVarId, Rc<dyn Constraint<'s, D> + 's>>;

/** The core struct used to contain and manage [value](crate::value) bindings.

An open [State] can be updated in a few different ways. Most update methods
return an `Option<State<D>>` to reflect the fact each new constraint
invalidate the state. This gives you the ability to quickly short circuit as
soon the state hits a dead end.

In general, it is most ergonomic to manipulate a state inside a function
that returns an `Option<State<D>>` to allow the use of the question mark
operator (Note that the [`.apply()`](State::apply()) function makes it easy
to do this).

```
use canrun::{State, val, var};
use canrun::example::I32;

fn my_fn<'a>() -> Option<State<'a, I32>> {
    let x = var();
    let y = var();
    let state: State<I32> = State::new();
    let maybe: Option<State<I32>> = state.unify(&val!(x), &val!(1));
    maybe?.unify(&val!(x), &val!(y))
}
assert!(my_fn().is_some());
```
*/
#[derive(Clone)]
pub struct State<'a, D: Domain<'a> + 'a> {
    domain: D,
    constraints: ConstraintFns<'a, D>,
    forks: im_rc::Vector<Rc<dyn Fork<'a, D> + 'a>>,
}

impl<'a, D: Domain<'a> + 'a> State<'a, D> {
    /**     Create a new, empty state.

    This often does not need to be used directly as you can
    [`.query()`](crate::goals::Goal::query()) a [`Goal`](crate::goals::Goal)
    directly, which handles the state creation internally.

    However, there are use cases for creating and managing a state
    independently of any goals.

    # Example:
    ```
    use canrun::{State, var};
    use canrun::example::I32;

    let state: State<I32> = State::new();
    ```
    */
    pub fn new() -> Self {
        State {
            domain: D::new(),
            constraints: MKMVMap::new(),
            forks: im_rc::Vector::new(),
        }
    }

    /** Apply an arbitrary function to a state.

    This is primarily a helper to make it easier to get into a function
    where you can use the question mark operator while applying multiple
    updates to a state.

    # Example:
    ```
    use canrun::{State, Query, val, var};
    use canrun::example::I32;

    let s: State<I32> = State::new();
    let x = var();
    let s = s.apply(|s| {
        s.unify(&val!(x), &val!(1))?
         .unify(&val!(1), &val!(x))
    });
    let results: Vec<i32> = s.query(x).collect();
    assert_eq!(results, vec![1]);
    ```
    */
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
            Some(fork) => Box::new(fork.fork(self).flat_map(State::iter_forks)),
        }
    }

    /** Recursively resolve a [`Val`](crate::value::Val) as far as the currently
    known variable bindings allow.

    This will return either the final [`Val::Resolved`] (if found) or the
    last [`Val::Var`] it attempted to resolve. It will not force
    [`forks`](State::fork()) to enumerate, so potential bindings are not
    considered.

    # Example:
    ```
    use canrun::{State, Query, val, var};
    use canrun::example::I32;

    # fn test() -> Option<()> {
    let state: State<I32> = State::new();

    let x = val!(var());
    assert_eq!(state.resolve_val(&x), &x);

    let state = state.unify(&x, &val!(1))?;
    assert_eq!(state.resolve_val(&x), &val!(1));
    # Some(())
    # }
    # test();
    ```
    */
    pub fn resolve_val<'r, T>(&'r self, val: &'r Val<T>) -> &'r Val<T>
    where
        T: Debug,
        D: DomainType<'a, T>,
    {
        self.domain.resolve(val)
    }

    /** Attempt to [unify](crate::unify::UnifyIn) two values with each other.

    If the unification fails, [`None`](std::option::Option::None) will be
    returned. [`Val::Var`]s will be checked against relevant
    [constraints](State::constrain), which can also cause a state to fail.

    # Examples:

    ```
    use canrun::{State, Query, val, var};
    use canrun::example::I32;

    let x = val!(var());

    let state: State<I32> = State::new();
    let state = state.unify(&x, &val!(1));
    assert!(state.is_some());
    ```

    ```
    # use canrun::{State, Query, val};
    # use canrun::example::I32;
    let state: State<I32> = State::new();
    let state = state.unify(&val!(1), &val!(2));
    assert!(state.is_none());
    ```
    */
    pub fn unify<T>(mut self, a: &Val<T>, b: &Val<T>) -> Option<Self>
    where
        T: UnifyIn<'a, D> + Debug,
        D: DomainType<'a, T>,
    {
        let a = self.resolve_val(a);
        let b = self.resolve_val(b);
        match (a, b) {
            (Resolved(a), Resolved(b)) => {
                let a = a.clone();
                let b = b.clone();
                UnifyIn::unify_resolved(self, a, b)
            }
            (Var(a), Var(b)) if a == b => Some(self),
            (Var(var), val) | (val, Var(var)) => {
                let key = *var;
                let value = val.clone();

                // TODO: Add occurs check?

                self.domain.update(key, value);

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

    /** Add a constraint to the store that can be reevaluated as variables are
    resolved.

    Some logic is not easy or even possible to express until the resolved
    values are available. `.constrain()` provides a low level way to run
    custom imperative code whenever certain bindings are updated.

    See the [`Constraint` trait](constraints::Constraint) for more
    information.
    */
    pub fn constrain(mut self, constraint: Rc<dyn Constraint<'a, D> + 'a>) -> Option<Self> {
        match constraint.attempt(&self) {
            Ok(resolve) => resolve(self),
            Err(watch) => {
                self.constraints.add(watch.0, constraint);
                Some(self)
            }
        }
    }

    /** Add a potential fork point to the state.

    If there are many possibilities for a certain value or set of values,
    this method allows you to add a [`Fork`] object that can enumerate those
    possible alternate states.

    While this is not quite as finicky as the
    [`Constraints`](State::constrain()), you still probably want to use the
    [`any`](crate::goals::any!) or [`either`](crate::goals::either()) goals.

    [Unification](State::unify()) is performed eagerly as soon as it is
    called. [Constraints](State::constrain()) are run as variables are
    resolved. Forking is executed lazily at the end, when
    [`.iter_resolved()`](crate::state::IterResolved::iter_resolved()) (or
    [`.query()](crate::Query::query())) is called.
    */
    pub fn fork(mut self, fork: Rc<dyn Fork<'a, D> + 'a>) -> Option<Self> {
        self.forks.push_back(fork);
        Some(self)
    }
}

/** Fork a [`State`] into zero or more alternate states.

Added to a [`State`] with [`.fork()`](crate::state::State::fork()).

# Example:
```
use canrun::{val, var, Fork, Query, State, StateIter, Val};
use canrun::example::I32;
use std::rc::Rc;

#[derive(Debug)]
struct Is1or2 {
    x: Val<i32>,
}

impl<'a> Fork<'a, I32> for Is1or2 {
    fn fork(&self, state: State<'a, I32>) -> StateIter<'a, I32> {
        let s1 = state.clone().unify(&self.x, &val!(1));
        let s2 = state.unify(&self.x, &val!(2));
        Box::new(s1.into_iter().chain(s2.into_iter()))
    }
}

# fn main() {
let x = var();
let state: State<I32> = State::new();
let state = state.fork(Rc::new(Is1or2 { x: val!(x) }));
let results: Vec<i32> = state.query(x).collect();
assert_eq!(results, vec![1, 2]);
# }
```
*/
pub trait Fork<'a, D: Domain<'a>>: Debug {
    /// Given a [`State`], return an iterator of states that result from the
    /// fork operation.
    fn fork(&self, state: State<'a, D>) -> StateIter<'a, D>;
}

#[cfg(test)]
mod test {
    use crate::example::I32;
    use crate::{val, var, Fork, Query, State, StateIter, Val};
    use std::rc::Rc;

    #[derive(Debug)]
    struct Is1or2 {
        x: Val<i32>,
    }

    impl<'a> Fork<'a, I32> for Is1or2 {
        fn fork(&self, state: State<'a, I32>) -> StateIter<'a, I32> {
            let s1 = state.clone().unify(&self.x, &val!(1));
            let s2 = state.unify(&self.x, &val!(2));
            Box::new(s1.into_iter().chain(s2.into_iter()))
        }
    }

    #[test]
    fn doctest() {
        let x = var();
        let state: State<I32> = State::new();

        let state = state.fork(Rc::new(Is1or2 { x: val!(x) }));
        let results: Vec<i32> = state.query(x).collect();
        assert_eq!(results, vec![1, 2]);
    }
}
