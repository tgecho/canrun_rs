use std::{fmt::Debug, hash::Hash};

use crate::{goals::Goal, Unify, Value};

use super::{subset, LMap};

/**
Assert that [`LMap`] `a` is a superset of [`LMap`] `b`.

This means that all of the keys in `b` unify with keys in `a` AND the
corresponding values also unify. This is the opposite of [`subset`](crate::lmap::subset).

# Example:
```
use canrun::{LVar, Query};
use canrun::lmap::{lmap, superset};

let x = LVar::new();
let goal = superset(lmap! {x => 2, 3 => 4}, lmap! {1 => 2});
let results: Vec<_> = goal.query(x).collect();
assert_eq!(results, vec![1]);
```
*/
pub fn superset<K, V, A, B>(a: A, b: B) -> impl Goal
where
    K: Unify + Eq + Hash + Debug,
    V: Unify + Debug,
    A: Into<Value<LMap<K, V>>>,
    B: Into<Value<LMap<K, V>>>,
{
    subset(b, a)
}

#[cfg(test)]
mod tests {
    use crate::lmap::{lmap, superset};
    use crate::{LVar, Query};

    #[test]
    fn succeeds() {
        let x = LVar::new();
        let goal = superset(lmap! {x => 2, 3 => 4}, lmap! {1 => 2});
        let results: Vec<_> = goal.query(x).collect();
        assert_eq!(results, vec![1]);
    }
}