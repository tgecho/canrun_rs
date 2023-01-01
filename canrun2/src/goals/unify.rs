use std::fmt::Debug;

use crate::core::Value;
use crate::core::{State, Unify};

use super::Goal;

#[derive(Debug)]
pub struct UnifyGoal<T: Unify> {
    a: Value<T>,
    b: Value<T>,
}

impl<T: Unify> Goal for UnifyGoal<T> {
    fn apply(&self, state: State) -> Option<State> {
        state.unify(&self.a, &self.b)
    }
}

pub fn unify<T, A, B>(a: A, b: B) -> UnifyGoal<T>
where
    T: Unify,
    A: Into<Value<T>>,
    B: Into<Value<T>>,
{
    UnifyGoal {
        a: a.into(),
        b: b.into(),
    }
}

#[cfg(test)]
mod tests {
    use crate::core::LVar;

    use super::*;

    #[test]
    fn deeply_nested_vars() {
        let x = LVar::new();
        let goal = unify(x, 1);
        let result = goal.apply(State::new());
        assert_eq!(result.unwrap().resolve(&x.into()), 1.into());
    }
}
