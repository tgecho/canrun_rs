//! A [`Vec`]-like data structure with [`Value`](crate::Value) values.

mod get;
mod member;
mod slice;
mod subset;

use crate::{
    core::{Reify, State, Unify, Value},
    LVarList, ReadyState,
};
pub use get::{get, Get};
pub use member::{member, Member};
pub use slice::{slice, Slice};
pub use subset::{subset, Subset};

use std::rc::Rc;

/// A [`Vec`]-like data structure with [`Value`](crate::Value) values.
///
/// Construct with the [`lvec!`](crate::lvec!) macro, or you can use the
/// `From<Vec<Value<T>>>` or `FromIterator<Value<T>>` trait implementations.
#[derive(Debug)]
pub struct LVec<T: Unify> {
    vec: Vec<Value<T>>,
}

impl<T: Unify> LVec<T> {
    /// Returns the number of elements in the [`LVec`].
    pub fn len(&self) -> usize {
        self.vec.len()
    }

    /// Returns true if the [`LVec`] contains no elements.

    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }
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
    fn reify_in(&self, state: &ReadyState) -> Result<Vec<T::Reified>, LVarList> {
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

impl<T: Unify> From<&[Value<T>]> for LVec<T> {
    fn from(slice: &[Value<T>]) -> Self {
        LVec {
            vec: slice.to_vec(),
        }
    }
}

impl<T: Unify> FromIterator<Value<T>> for LVec<T> {
    fn from_iter<I: IntoIterator<Item = Value<T>>>(iter: I) -> Self {
        LVec {
            vec: iter.into_iter().collect(),
        }
    }
}

impl<T: Unify> FromIterator<T> for LVec<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        LVec {
            vec: iter.into_iter().map(Value::new).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{core::LVar, core::Query, goals::unify, lvec::LVec, Value};

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

    #[test]
    fn is_empty() {
        let empty: LVec<usize> = lvec![];
        assert!(empty.is_empty());
    }

    #[test]
    fn from_iter_value_t() {
        let from_iter: LVec<usize> = (1..3).collect();
        assert_eq!(from_iter.vec, vec![Value::new(1), Value::new(2)]);
    }

    #[test]
    fn from_iter_t() {
        let from_iter: LVec<usize> = (1..3).map(Value::new).collect();
        assert_eq!(from_iter.vec, vec![Value::new(1), Value::new(2)]);
    }
}
