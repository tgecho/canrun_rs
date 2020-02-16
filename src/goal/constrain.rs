use crate::{Can, CanT, Goal};

#[derive(Clone, PartialEq, Debug)]
pub struct Constraint<T: CanT> {
    pub left: Can<T>,
    pub right: Can<T>,
    pub func: fn(T, T) -> bool,
}

pub fn constrain<T: CanT>(a: Can<T>, b: Can<T>, func: fn(T, T) -> bool) -> Goal<T> {
    Goal::Constrain(Constraint {
        left: a,
        right: b,
        func,
    })
}
