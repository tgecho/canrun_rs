use std::{fmt::Debug, hash::Hash};

use crate::{goals::Goal, lmap, Unify, Value};

use super::{subset, LMap};

/**
Assert that the a given key and value combination can be found in an
[`LMap`]

This is essentially a single key case of [`subset`](crate::lmap::subset::subset).

# Example:
```
use canrun::{LVar, Query};
use canrun::lmap::{lmap, get};

let x = LVar::new();
let goal = get(1, x, lmap! {1 => 2});
let results: Vec<_> = goal.query(x).collect();
```
*/
pub fn get<K, V>(
    key: impl Into<Value<K>>,
    value: impl Into<Value<V>>,
    b: impl Into<Value<LMap<K, V>>>,
) -> impl Goal
where
    K: Unify + Eq + Hash + Debug,
    V: Unify + Debug,
{
    subset(lmap! {key => value}, b)
}

#[cfg(test)]
mod tests {
    use crate::lmap::{get, lmap};
    use crate::{LVar, Query};

    #[test]
    fn succeeds() {
        let x = LVar::new();
        let goal = get(1, x, lmap! {1 => 2});
        let results: Vec<_> = goal.query(x).collect();
        assert_eq!(results, vec![2]);
    }
}
