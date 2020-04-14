use super::super::state::{Constraint, State};
use crate::domains::example::I32;
use crate::domains::DomainType;
use crate::goal::custom;
use crate::goal::unify;
use crate::goal::Goal;
use crate::util;
use crate::value::{val, var, IntoVal};
use std::fmt::Debug;
use std::rc::Rc;

pub(crate) fn assert<'a, T, V, D, F>(
    val: V,
    func: F,
) -> Rc<dyn Fn(State<'a, D>) -> Constraint<State<'a, D>> + 'a>
where
    T: Debug + 'a,
    V: IntoVal<T> + Clone + 'a,
    D: DomainType<'a, T> + 'a,
    F: Fn(&T) -> bool + 'a,
{
    Rc::new(move |s| {
        let val = val.clone().into_val();
        match s.resolve_val(&val).resolved() {
            Ok(x) => Constraint::Done(if func(x) { Some(s) } else { None }),
            Err(x) => Constraint::on_1(s, x),
        }
    })
}

#[test]
fn basic_constrain_succeeds() {
    let x = var();
    let goals: Vec<Goal<I32>> = vec![
        unify(2, x),
        custom(|s| s.constrain(assert(x, |x| x > &1))),
        custom(|s| s.constrain(assert(x, |x| x > &0))),
    ];
    util::assert_permutations_resolve_to(goals, x, vec![2]);
}

#[test]
fn basic_constrain_fails() {
    let x = var();
    let goals: Vec<Goal<I32>> = vec![
        unify(&val!(2), x.clone()),
        custom(|s| s.constrain(assert(x.clone(), |x| x > &1))),
        custom(|s| s.constrain(assert(x, |x| x > &3))),
    ];
    util::assert_permutations_resolve_to(goals, x, vec![]);
}

#[test]
fn unsatisfied_constrain_fails() {
    let x = var();
    let y = var();
    let goals: Vec<Goal<I32>> = vec![
        unify(&val!(1), x),
        custom(|s| s.constrain(assert(y, |y| y < &3))),
    ];
    util::assert_permutations_resolve_to(goals.clone(), x, vec![]);
    util::assert_permutations_resolve_to(goals, y, vec![]);
}
