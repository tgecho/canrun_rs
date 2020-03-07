use super::super::domain::{Domain, DomainType, Just};
use super::super::goal::Goal;
use super::super::state::{State, WatchResult};
use super::super::val::{val, var, Val};
use super::utils;
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
fn basic_watch_succeeds() {
    let x = var();
    let goals = vec![
        Goal::thunk(|s| s.unify(val(2), x.clone())),
        Goal::thunk(|s| s.watch(assert(x.clone(), |x| x > &1))),
    ];
    for goals in utils::all_permutations(goals) {
        let s: State<Just<i32>> = State::new();
        let result = Goal::All(goals).apply(s);
        assert!(result.is_ok());
    }
}

#[test]
fn basic_watch_fails() {
    let x = var();
    let goals = vec![
        Goal::thunk(|s| s.unify(val(2), x.clone())),
        Goal::thunk(|s| s.watch(assert(x.clone(), |x| x > &2))),
    ];
    for goals in utils::all_permutations(goals) {
        let s: State<Just<i32>> = State::new();
        let result = Goal::All(goals).apply(s);
        assert!(result.is_err());
    }
}
