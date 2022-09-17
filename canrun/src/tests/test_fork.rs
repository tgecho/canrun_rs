use super::super::state::{Fork, IterResolved, State};
use crate::domains::Domain;
use crate::example::I32;
use crate::value::val;
use std::fmt;
use std::rc::Rc;

struct Either<'a, D: Domain<'a>>(
    Rc<dyn Fn(State<'a, D>) -> Option<State<'a, D>> + 'a>,
    Rc<dyn Fn(State<'a, D>) -> Option<State<'a, D>> + 'a>,
);

impl<'a, D: Domain<'a>> fmt::Debug for Either<'a, D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Either")
    }
}

impl<'a, D> Fork<'a, D> for Either<'a, D>
where
    D: Domain<'a>,
{
    fn fork(&self, state: State<'a, D>) -> crate::state::StateIter<'a, D> {
        let a = (self.0)(state.clone()).into_iter();
        let b = (self.1)(state).into_iter();
        Box::new(a.chain(b))
    }
}

#[test]
fn basic_fork_first_success() {
    let state: State<I32> = State::new();
    let state = state.fork(Rc::new(Either(
        Rc::new(|s| s.unify(&val!(2), &val!(2))),
        Rc::new(|s| s.unify(&val!(1), &val!(2))),
    )));
    assert_eq!(1, state.unwrap().iter_resolved().count());
}

#[test]
fn basic_fork_second_success() {
    let state: State<I32> = State::new();
    let state = state.fork(Rc::new(Either(
        Rc::new(|s| s.unify(&val!(1), &val!(2))),
        Rc::new(|s| s.unify(&val!(2), &val!(2))),
    )));
    assert_eq!(1, state.iter_resolved().count());
}

#[test]
fn basic_fork_both_success() {
    let state: State<I32> = State::new();
    let state = state.fork(Rc::new(Either(
        Rc::new(|s| s.unify(&val!(1), &val!(1))),
        Rc::new(|s| s.unify(&val!(2), &val!(2))),
    )));
    assert_eq!(2, state.iter_resolved().count());
}
