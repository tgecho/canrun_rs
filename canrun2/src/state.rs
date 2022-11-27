use std::{any::Any, fmt::Debug, iter::once, rc::Rc};

use crate::lvar::{LVar, LVarId};

/// The possible states a value can be in.
pub enum Val<T: Debug + ?Sized> {
    /// A [logical variable](LVar).
    Var(LVar<T>),
    /** A resolved value.

    When a state is split into an arbitrary number of [resolved
    states](crate::state::ResolvedState), some of the internal data
    structures often need to be cloned. In an attempt to avoid unnecessary
    cloning of every value in the state, we wrap it in an [Rc] so that
    references can be shared.
    */
    Resolved(Rc<T>),
}
use Val::{Resolved, Var};

pub type StateIter<'s> = Box<dyn Iterator<Item = State<'s>> + 's>;

pub trait Fork<'a>: Debug {
    /// Given a [`State`], return an iterator of states that result from the
    /// fork operation.
    fn fork(&self, state: State<'a>) -> StateIter<'a>;
}

#[derive(Clone)]
pub struct State<'a> {
    values: im_rc::HashMap<LVarId, Rc<dyn Any>>,
    forks: im_rc::Vector<Rc<dyn Fork<'a> + 'a>>,
}

pub trait Unify<'a>: Sized + Debug + 'static {
    /** Attempt to unify two fully resolved values.

    This function accepts `Rc<T>`s to simplify the borrow checking. The
    `Option<_>` allows recursive unification of structures that hold
    additional values.
    */
    fn unify_resolved(state: State<'a>, a: Rc<Self>, b: Rc<Self>) -> Option<State<'a>>;
}

impl<'a> State<'a> {
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
            values: im_rc::HashMap::new(),
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

    fn iter_forks(mut self) -> StateIter<'a> {
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
        T: Unify<'r>,
    {
        match val {
            Val::Var(var) => {
                let resolved = self.values.get(&var.id).and_then(|any| any.downcast_ref());
                match resolved {
                    Some(Val::Var(found)) if found == var => val,
                    Some(found) => self.resolve_val(found),
                    _ => val,
                }
            }
            value => value,
        }
    }

    /** Attempt to [unify](crate::unify::Unify) two values with each other.

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
        T: Unify<'a>,
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

                self.values.insert(key.id, Rc::new(value));

                // check constraints matching newly assigned lvar
                // if let Some(constraints) = self.constraints.extract(&key.id) {
                //     constraints
                //         .into_iter()
                //         .try_fold(self, |state, func| state.constrain(func))
                // } else {
                Some(self)
                // }
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
    pub fn fork(mut self, fork: Rc<dyn Fork<'a> + 'a>) -> Option<Self> {
        self.forks.push_back(fork);
        Some(self)
    }
}
