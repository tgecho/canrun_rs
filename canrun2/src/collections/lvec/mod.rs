use std::rc::Rc;

use crate::{
    core::{Reify, State, Unify},
    value::Value,
};

/// A [`Vec`]-like data structure with [`LVar`](crate::value::LVar) values.
#[derive(Debug, Default)]
pub struct LVec<T: Unify> {
    vec: Vec<Value<T>>,
}

#[macro_export]
macro_rules! lvec {
    ($($item:expr),* $(,)?) => {
        {
            let mut lv = $crate::collections::lvec::LVec::new();
            $(lv.push($item.into());)*
            lv
        }
    };
}

impl<T: Unify> LVec<T> {
    pub fn new() -> Self {
        LVec { vec: Vec::new() }
    }

    pub fn push(&mut self, value: Value<T>) {
        self.vec.push(value);
    }
}

impl<T: Unify> Unify for LVec<T> {
    fn unify(state: State, a: Rc<Self>, b: Rc<Self>) -> Option<State> {
        if a.vec.len() == b.vec.len() {
            a.vec
                .iter()
                .zip(b.vec.iter())
                .try_fold(state, |s: State, (a, b)| s.unify(a, b))
        } else {
            None
        }
    }
}

impl<T: Unify + Reify> Reify for LVec<T> {
    type Concrete = Vec<T::Concrete>;
    fn reify_in(&self, state: &State) -> Option<Vec<T::Concrete>> {
        self.vec
            .iter()
            .map(|v: &Value<T>| v.reify_in(state))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        core::{Reify, State, StateIterator},
        goals::{unify::unify, Goal},
        value::{LVar, Value},
    };

    #[test]
    fn succeeds() {
        let x = LVar::new();
        let goal = unify(lvec![x, 2], lvec![1, 2]);
        let result = goal.apply(State::new());
        assert_eq!(result.unwrap().resolve(&x.into()), 1.into());
    }

    #[test]
    fn reify_vec() {
        let x = Value::new(lvec![1, 2]);
        State::new().into_states().for_each(|state| {
            assert_eq!(x.reify_in(&state), Some(vec![1, 2]));
        });
    }
}
