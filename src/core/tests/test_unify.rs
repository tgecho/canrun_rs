use super::super::state::State;
use crate::domain::one::OfOne;
use crate::query::{QueryState, StateQuery};
use crate::value::{val, var};

#[test]
fn basic_unifying_literals() {
    let s: State<OfOne<i32>> = State::new();
    assert!(s.clone().unify(val(1), val(1)).is_some());
    assert!(s.clone().unify(val(1), val(2)).is_none());
}

#[test]
fn basic_unifying_vars() {
    let s: State<OfOne<i32>> = State::new();
    assert!(s.clone().unify(var(), val(1)).is_some());
    assert!(s.clone().unify(val(1), var()).is_some());
}

#[test]
fn unifying_var_success() {
    let s: State<OfOne<i32>> = State::new();
    let x = &var();
    let s = s.apply(|s| s.unify(x.clone(), val(1))?.unify(val(1), x.clone()));
    let results: Vec<i32> = s.query(x).collect();
    assert_eq!(results, vec![1]);
}

#[test]
fn unifying_var_fails() {
    let s: State<OfOne<i32>> = State::new();
    let s = s.apply(|s| {
        let x = var();
        s.unify(x.clone(), val(1))?.unify(val(2), x)
    });
    assert!(s.is_none());
}
