use super::super::state::State;
use crate::example::I32;
use crate::value::{val, var, IntoVal};
use crate::Query;

#[test]
fn basic_unifying_literals() {
    let s: State<I32> = State::new();
    assert!(s.clone().unify(&val!(1), &val!(1)).is_some());
    assert!(s.clone().unify(&val!(1), &val!(2)).is_none());
}

#[test]
fn basic_unifying_vars() {
    let s: State<I32> = State::new();
    let x = var();

    assert_eq!(
        &val!(1),
        s.clone()
            .unify(&val!(x), &val!(1))
            .unwrap()
            .resolve_val(&val!(x))
    );
    assert_eq!(
        &val!(1),
        s.clone()
            .unify(&val!(1), &val!(x))
            .unwrap()
            .resolve_val(&val!(x))
    );
}

#[test]
fn unifying_var_success() {
    let s: State<I32> = State::new();
    let x = var();
    let s = s.apply(|s| s.unify(&val!(x), &val!(1))?.unify(&val!(1), &val!(x)));
    let results: Vec<i32> = s.query(x).collect();
    assert_eq!(results, vec![1]);
}

#[test]
fn unifying_var_fails() {
    let s: State<I32> = State::new();
    let s = s.apply(|s| {
        let x = var();
        s.unify(&x.into_val(), &val!(1))?
            .unify(&val!(2), &x.into_val())
    });
    assert!(s.is_none());
}

#[test]
fn multipart_unifying_vars() {
    let s: State<I32> = State::new();
    let x = var();
    let y = var();
    let s = s.apply(|s| {
        let s = s.unify(&val!(x), &val!(y))?;
        s.unify(&val!(1), &val!(y))
    });
    let results: Vec<_> = s.query((x, y)).collect();
    assert_eq!(results, vec![(1, 1)]);
}
