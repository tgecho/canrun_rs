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
pub fn get<K, KV, V, VV, B>(key: KV, value: VV, b: B) -> impl Goal
where
    K: Unify + Eq + Hash + Debug,
    KV: Into<Value<K>>,
    V: Unify + Debug,
    VV: Into<Value<V>>,
    B: Into<Value<LMap<K, V>>>,
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
