use super::super::domain::{DomainType, Just};
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
        Ok(x) => WatchResult::Done(if func(x) { Some(s) } else { None }),
        Err(x) => WatchResult::Waiting(s, vec![x]),
    })
}

#[test]
fn basic_watch_succeeds() {
    let x = var();
    let goals: Vec<Goal<Just<i32>>> = vec![
        Goal::thunk(|s| s.unify(val(2), x.clone())),
        Goal::thunk(|s| s.watch(assert(x.clone(), |x| x > &1))),
    ];
    utils::all_permutations_resolve_to(goals, &x, vec![2]);
}

#[test]
fn basic_watch_fails() {
    let x = var();
    let goals: Vec<Goal<Just<i32>>> = vec![
        Goal::thunk(|s| s.unify(val(2), x.clone())),
        Goal::thunk(|s| s.watch(assert(x.clone(), |x| x > &2))),
    ];
    utils::all_permutations_resolve_to(goals, &x, vec![]);
}
