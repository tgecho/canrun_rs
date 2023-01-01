//! A [`Vec`]-like data structure with [`Value`](crate::Value) values.

mod get;
mod member;

use crate::core::{Reify, State, Unify, Value};
pub use get::{get, Get};
pub use member::{member, Member};

use std::rc::Rc;

/// A [`Vec`]-like data structure with [`Value`](crate::Value) values.
///
/// Construct with the [`lvec!`](crate::lvec!) macro, or you can use the
/// `From<Vec<Value<T>>>` or `FromIterator<Value<T>>` trait implementations.
#[derive(Debug)]
pub struct LVec<T: Unify> {
    vec: Vec<Value<T>>,
}

/** Create an [`LVec<T>`](crate::collections::lvec::LVec) with automatic `Into<Value<T>>` conversion.

The primary benefit is that it allows freely mixing `T`, [`Value<T>`](crate::Value) and
[`LVar<T>`](crate::LVar) without needing to do manual conversion.
*/
#[macro_export]
macro_rules! lvec {
    ($($item:expr),* $(,)?) => {
        {
            let vec = vec![$($item.into(),)*];
            $crate::collections::lvec::LVec::from(vec)
        }
    };
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

impl<T: Unify> From<Vec<Value<T>>> for LVec<T> {
    fn from(vec: Vec<Value<T>>) -> Self {
        LVec { vec }
    }
}

impl<T: Unify> FromIterator<Value<T>> for LVec<T> {
    fn from_iter<I: IntoIterator<Item = Value<T>>>(iter: I) -> Self {
        LVec {
            vec: iter.into_iter().collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{core::LVar, core::Query, goals::unify};

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
