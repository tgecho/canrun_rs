use super::super::state::{IterResolved, State, StateIter};
use crate::domain::{one::OfOne, Domain};
use crate::value::val;
use std::rc::Rc;

fn either<'a, D, A, B>(a: A, b: B) -> Rc<dyn Fn(State<'a, D>) -> StateIter<'a, D> + 'a>
where
    D: Domain<'a> + 'a,
    A: Fn(State<'a, D>) -> Option<State<'a, D>> + 'a,
    B: Fn(State<'a, D>) -> Option<State<'a, D>> + 'a,
{
    Rc::new(move |s| {
        let a = a(s.clone()).into_iter();
        let b = b(s).into_iter();
        Box::new(a.chain(b))
    })
}

#[test]
fn basic_fork_first_success() {
    let state: State<OfOne<i32>> = State::new();
    let state = state.fork(either(
        |s| s.unify(val(2), val(2)),
        |s| s.unify(val(1), val(2)),
    ));
    let results: Vec<_> = state.resolved_iter().collect();
    assert_eq!(1, results.len());
}

#[test]
fn basic_fork_second_success() {
    let state: State<OfOne<i32>> = State::new();
    let state = state.fork(either(
        |s| s.unify(val(1), val(2)),
        |s| s.unify(val(2), val(2)),
    ));
    assert_eq!(1, state.resolved_iter().count());
}

#[test]
fn basic_fork_both_success() {
    let state: State<OfOne<i32>> = State::new();
    let state = state.fork(either(
        |s| s.unify(val(1), val(1)),
        |s| s.unify(val(2), val(2)),
    ));
    assert_eq!(2, state.resolved_iter().count());
}
