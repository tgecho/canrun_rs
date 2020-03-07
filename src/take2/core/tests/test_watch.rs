use super::super::domain::{DomainType, Just};
use super::super::state::{State, WatchResult};
use super::super::val::{val, var, Val};
use std::rc::Rc;

fn assert<'a, T, D, F>(
    val: Val<T>,
    func: F,
) -> Rc<dyn Fn(State<'a, D>) -> WatchResult<State<'a, D>> + 'a>
where
    T: 'a,
    D: DomainType<T> + 'a,
    F: Fn(&T) -> bool + 'a,
{
    Rc::new(move |s| match s.resolve(&val).resolved() {
        Ok(x) => WatchResult::Done(if func(x) { Ok(s) } else { Err(s) }),
        Err(x) => WatchResult::Waiting(s, vec![x]),
    })
}

#[test]
fn basic_watch_after_succeeds() {
    let s: State<Just<i32>> = State::new();
    let s = s.apply(|s| {
        let x = var();
        s.unify(val(2), x.clone())?.watch(assert(x, |x| x > &1))
    });
    assert!(s.is_ok());
}

#[test]
fn basic_watch_after_fails() {
    let s: State<Just<i32>> = State::new();
    let s = s.apply(|s| {
        let x = var();
        s.unify(val(2), x.clone())?.watch(assert(x, |x| x < &1))
    });
    assert!(s.is_err());
}

#[test]
fn basic_watch_before_succeeds() {
    let s: State<Just<i32>> = State::new();
    let s = s.apply(|s| {
        let x = var();
        s.watch(assert(x.clone(), |x| x > &1))?.unify(val(2), x)
    });
    assert!(s.is_ok());
}

#[test]
fn basic_watch_before_fails() {
    let s: State<Just<i32>> = State::new();
    let s = s.apply(|s| {
        let x = var();
        s.watch(assert(x.clone(), |x| x < &1))?.unify(val(2), x)
    });
    assert!(s.is_err());
}
