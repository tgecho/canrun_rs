use super::super::state::{State, WatchResult};
use super::util;
use crate::domain::{DomainType, Just};
use crate::goal::custom::custom;
use crate::goal::unify::unify;
use crate::goal::Goal;
use crate::value::{val, var, Val};
use std::rc::Rc;

pub(crate) fn assert<'a, T, D, F>(
    val: Val<T>,
    func: F,
) -> Rc<dyn Fn(State<'a, D>) -> WatchResult<State<'a, D>> + 'a>
where
    T: 'a,
    D: DomainType<'a, T> + 'a,
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
        unify(val(2), x.clone()),
        custom(|s| s.watch(assert(x.clone(), |x| x > &1))),
    ];
    util::all_permutations_resolve_to(goals, &x, vec![2]);
}

#[test]
fn basic_watch_fails() {
    let x = var();
    let goals: Vec<Goal<Just<i32>>> = vec![
        unify(val(2), x.clone()),
        custom(|s| s.watch(assert(x.clone(), |x| x > &2))),
    ];
    util::all_permutations_resolve_to(goals, &x, vec![]);
}
