mod get;
mod member;

use crate::core::{Reify, State, Unify, Value};
pub use get::{get, Get};
pub use member::{member, Member};

use std::rc::Rc;

/// A [`Vec`]-like data structure with [`LVar`](crate::core::LVar) values.
#[derive(Debug, Default)]
pub struct LVec<T: Unify> {
    vec: Vec<Value<T>>,
}

/** Create an [`LVec<T>`](crate::collections::lvec::LVec) with automatic `Into<Value<T>>` conversion.

The primary benefit is that it allows freely mixing resolved values and
[`LVar`s](crate::core::LVar).
*/
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
    type Reified = Vec<T::Reified>;
    fn reify_in(&self, state: &State) -> Option<Vec<T::Reified>> {
        self.vec
            .iter()
            .map(|v: &Value<T>| v.reify_in(state))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::{core::LVar, core::Query, goals::unify::unify};

    #[test]
    fn succeeds() {
        let x = LVar::new();
        let goal = unify(lvec![x, 2], lvec![1, 2]);
        assert_eq!(goal.query(x).collect::<Vec<_>>(), vec![1]);
    }

    #[test]
    fn fails() {
        let x = LVar::new();
        let goal = unify(lvec![x, 1], lvec![1, 2]);
        assert_eq!(goal.query(x).count(), 0);
    }
}
