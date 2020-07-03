//! A [`Vec`]-like data structure with [`LVar`](crate::LVar) values.

mod member;
mod subset;

pub use member::member;
pub use subset::subset;

use crate::{DomainType, IntoVal, ReifyIn, ResolvedState, State, UnifyIn, Val};
use std::fmt::Debug;
use std::rc::Rc;

/// A [`Vec`]-like data structure with [`LVar`](crate::value::LVar) values.
#[derive(Debug, Clone)]
pub struct LVec<T: Debug> {
    vec: Vec<Val<T>>,
}

impl<V: Debug> LVec<V> {
    /// Create a new [`LVec`] value.
    ///
    /// You may also be interested in the [`lvec!`] macro.
    ///
    /// # Example:
    /// ```
    /// use canrun::lvec::LVec;
    ///
    /// let map: LVec<i32> = LVec::new();
    /// ```
    pub fn new() -> Self {
        LVec { vec: Vec::new() }
    }

    /// Get the number of elements in the [LVec].
    ///
    /// # Example:
    /// ```
    /// use canrun::lvec::{LVec, lvec};
    ///
    /// let map: LVec<i32> = lvec![1, 2];
    /// assert_eq!(map.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        self.vec.len()
    }

    /// Add a value to an existing [`LVec`].
    ///
    /// # Example:
    /// ```
    /// use canrun::lvec::LVec;
    ///
    /// let mut map: LVec<i32> = LVec::new();
    /// map.push(1);
    /// ```
    pub fn push<Vi>(&mut self, value: Vi)
    where
        Vi: IntoVal<V>,
    {
        self.vec.push(value.into_val());
    }
}

/// Create an [`LVec<T>`](lvec::LVec) with automatic value [`IntoVal`
/// wrapping](crate::IntoVal).
///
/// The primary benefit is that it allows freely mixing resolved values and
/// [`LVar`s](crate::LVar).
///
/// # Example:
/// ```
/// use canrun::var;
/// use canrun::collections::lvec::{lvec, LVec};
/// let x = var();
/// let map: LVec<i32> = lvec![x, 1, 2];
/// ```
#[macro_export]
macro_rules! lvec {
    ($($item:expr),* $(,)?) => {
        {
            let mut lv = $crate::lvec::LVec::new();
            $(lv.push($crate::value::IntoVal::into_val($item));)*
            lv
        }
    };
}

#[doc(inline)]
pub use lvec;

impl<'a, T, D> UnifyIn<'a, D> for LVec<T>
where
    T: UnifyIn<'a, D>,
    D: DomainType<'a, T> + DomainType<'a, LVec<T>>,
{
    fn unify_resolved(state: State<'a, D>, a: Rc<LVec<T>>, b: Rc<LVec<T>>) -> Option<State<'a, D>> {
        if a.vec.len() == b.vec.len() {
            a.vec
                .iter()
                .zip(b.vec.iter())
                .try_fold(state, |s: State<'a, D>, (a, b)| s.unify(a, b))
        } else {
            None
        }
    }
}

impl<'a, D, T> ReifyIn<'a, D> for LVec<T>
where
    T: ReifyIn<'a, D> + Debug,
    D: DomainType<'a, T> + 'a,
{
    type Reified = Vec<T::Reified>;
    fn reify_in(&self, state: &ResolvedState<D>) -> Option<Self::Reified> {
        self.vec.iter().map(|v: &Val<T>| state.reify(v)).collect()
    }
}

impl<'a, T, I, IV> From<I> for LVec<T>
where
    T: Debug,
    IV: IntoVal<T>,
    I: IntoIterator<Item = IV>,
{
    fn from(i: I) -> Self {
        LVec {
            vec: i.into_iter().map(|t| t.into_val()).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::example::Collections;
    use crate::{unify, util, val, var, Goal, IterResolved, ReifyIn, ResolvedState, State};

    #[test]
    fn succeeds() {
        let x = var();
        let goals: Vec<Goal<Collections>> = vec![unify(x, lvec![1, 2]), unify(x, lvec![1, 2])];
        util::assert_permutations_resolve_to(goals, x, vec![vec![1, 2]]);
    }

    #[test]
    fn fails() {
        let x = var();
        let goals: Vec<Goal<Collections>> = vec![unify(x, lvec![1, 3]), unify(x, lvec![1, 2])];
        util::assert_permutations_resolve_to(goals, x, vec![]);
    }

    #[test]
    fn nested_var() {
        let x = var();
        let y = var::<i32>();
        let goals: Vec<Goal<Collections>> = vec![unify(x, lvec![1, y]), unify(x, lvec![1, 2])];
        util::assert_permutations_resolve_to(goals, y, vec![2]);
    }

    #[test]
    fn reify_vec() {
        let x = val!(lvec![1, 2]);
        State::new()
            .iter_resolved()
            .for_each(|state: ResolvedState<Collections>| {
                assert_eq!(x.reify_in(&state), Some(vec![1, 2]));
            });
    }
}
