use std::{fmt::Debug, hash::Hash};

use crate::{
    goals::{custom, project_2, Goal},
    Unify, Value,
};

use super::{unify_entries, LMap};

/** Assert that [`LMap`] `a` is a subset of [`LMap`] `b`.

This means that all of the keys in `a` unify with keys in `b` AND the
corresponding values also unify. This is the opposite of [`superset`](crate::lmap::superset).

# Example:
```
use canrun2::{LVar, Query};
use canrun2::lmap::{lmap, subset};

let x = LVar::new();
let goal = subset(lmap! {x => 2}, lmap! {1 => 2, 3 => 4});
let results: Vec<_> = goal.query(x).collect();
assert_eq!(results, vec![1]);
```
*/
pub fn subset<K, V, A, B>(a: A, b: B) -> impl Goal
where
    K: Unify + Eq + Hash + Debug,
    V: Unify + Debug,
    A: Into<Value<LMap<K, V>>>,
    B: Into<Value<LMap<K, V>>>,
{
    project_2(a, b, |a, b| {
        Box::new(custom(move |state| {
            unify_entries(state, a.clone(), b.clone())
        }))
    })
}
