use crate::take2::core::domain::Just;
use crate::take2::core::state::State;
use crate::take2::core::val::{val, var};

#[test]
fn basic_unifying_literals() {
    let s: State<Just<i32>> = State::new();
    assert!(s.clone().unify(val(1), val(1)).is_ok());
    assert!(s.clone().unify(val(1), val(2)).is_err());
}

#[test]
fn basic_unifying_vars() {
    let s: State<Just<i32>> = State::new();
    assert!(s.clone().unify(var(), val(1)).is_ok());
    assert!(s.clone().unify(val(1), var()).is_ok());
}

#[test]
fn unifying_var_success() {
    let s: State<Just<i32>> = State::new();
    let s = s.apply(|s| {
        let x = var();
        s.unify(x.clone(), val(1))?.unify(val(1), x)
    });
    assert!(s.is_ok());
}

#[test]
fn unifying_var_fails() {
    let s: State<Just<i32>> = State::new();
    let s = s.apply(|s| {
        let x = var();
        s.unify(x.clone(), val(1))?.unify(val(2), x)
    });
    assert!(s.is_err());
}
