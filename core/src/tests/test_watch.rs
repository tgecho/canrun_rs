use super::super::state::{State, Watch};
use crate::domain::DomainType;
use crate::goal::custom;
use crate::goal::unify;
use crate::goal::Goal;
use crate::tests::domains::Numbers;
use crate::util;
use crate::value::{val, var, IntoVal};
use std::rc::Rc;

pub(crate) fn assert<'a, T, V, D, F>(
    val: V,
    func: F,
) -> Rc<dyn Fn(State<'a, D>) -> Watch<State<'a, D>> + 'a>
where
    T: 'a,
    V: IntoVal<T> + Clone + 'a,
    D: DomainType<'a, T> + 'a,
    F: Fn(&T) -> bool + 'a,
{
    Rc::new(move |s| {
        let val = val.clone().into_val();
        match s.resolve_val(&val).resolved() {
            Ok(x) => Watch::Done(if func(x) { Some(s) } else { None }),
            Err(x) => Watch::watch(s, x),
        }
    })
}

#[test]
fn basic_watch_succeeds() {
    let x = var();
    let goals: Vec<Goal<Numbers>> = vec![
        unify(2, x),
        custom(|s| s.watch(assert(x, |x| x > &1))),
        custom(|s| s.watch(assert(x, |x| x > &0))),
    ];
    util::all_permutations_resolve_to(goals, x, vec![2]);
}

#[test]
fn basic_watch_fails() {
    let x = var();
    let goals: Vec<Goal<Numbers>> = vec![
        unify(val(2), x.clone()),
        custom(|s| s.watch(assert(x.clone(), |x| x > &1))),
        custom(|s| s.watch(assert(x, |x| x > &3))),
    ];
    util::all_permutations_resolve_to(goals, x, vec![]);
}
