use std::fmt::Debug;

use crate::core::{State, Unify as UnifyTrait};
use crate::value::Value;

use super::Goal;

#[derive(Debug)]
pub struct Unify<T: UnifyTrait> {
    a: Value<T>,
    b: Value<T>,
}

impl<T: UnifyTrait> Unify<T> {
    pub fn new(a: Value<T>, b: Value<T>) -> Self {
        Unify { a, b }
    }
}

impl<T: UnifyTrait> Goal for Unify<T> {
    fn apply(&self, state: State) -> Option<State> {
        state.unify(&self.a, &self.b)
    }
}

pub fn unify<T, A, B>(a: A, b: B) -> Unify<T>
where
    T: UnifyTrait,
    A: Into<Value<T>>,
    B: Into<Value<T>>,
{
    Unify {
        a: a.into(),
        b: b.into(),
    }
}

#[cfg(test)]
mod tests {
    use crate::value::LVar;

    use super::*;

    #[test]
    fn deeply_nested_vars() {
        let x = LVar::new();
        let goal = unify(x, 1);
        let result = goal.apply(State::new());
        assert_eq!(result.unwrap().resolve(&x.into()), 1.into());
    }
}
