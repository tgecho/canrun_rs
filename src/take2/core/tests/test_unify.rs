use super::super::domain::Just;
use super::super::state::{run, State};
use super::super::val::{val, var};

fn results<'a, F>(func: F) -> Vec<State<'a, Just<i32>>>
where
    F: Fn(State<Just<i32>>) -> Result<State<Just<i32>>, State<Just<i32>>>,
{
    run(func).collect()
}

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
