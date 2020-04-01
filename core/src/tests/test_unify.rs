use super::super::state::State;
use crate::query::StateQuery;
use crate::tests::domains::Numbers;
use crate::value::{val, var};

#[test]
fn basic_unifying_literals() {
    let s: State<Numbers> = State::new();
    assert!(s.clone().unify(val(1), val(1)).is_some());
    assert!(s.clone().unify(val(1), val(2)).is_none());
}

#[test]
fn basic_unifying_vars() {
    let s: State<Numbers> = State::new();
    assert!(s.clone().unify(var(), 1).is_some());
    assert!(s.clone().unify(1, var()).is_some());
}

#[test]
fn unifying_var_success() {
    let s: State<Numbers> = State::new();
    let x = var();
    let s = s.apply(|s| s.unify(x, 1)?.unify(1, x));
    let results: Vec<i32> = s.query(x).collect();
    assert_eq!(results, vec![1]);
}

#[test]
fn unifying_var_fails() {
    let s: State<Numbers> = State::new();
    let s = s.apply(|s| {
        let x = var();
        s.unify(x.clone(), val(1))?.unify(val(2), x)
    });
    assert!(s.is_none());
}
