//! A [`HashMap`](std::collections::HashMap)-like data structure with
//! [`LVar`](crate::LVar) keys and values.
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;

mod get;
mod subset;
mod superset;

pub use get::get;
pub use subset::subset;
pub use superset::superset;

/// A [`HashMap`](std::collections::HashMap)-like data structure with
/// [`LVar`](crate::LVar) keys and values.
#[derive(Clone, Default, Debug)]
pub struct LMap<K: Unify + Eq + Hash + Debug, V: Unify + Debug> {
    map: HashMap<Value<K>, Value<V>>,
}

impl<K: Unify + Eq + Hash + Debug, V: Unify + Debug> LMap<K, V> {
    /** Create a new [`LMap`] value.

    You may also be interested in the [`lmap!`] macro.

    # Example:
    ```
    use canrun::lmap::LMap;

    let map: LMap<i32, i32> = LMap::new();
    ```
    */
    pub fn new() -> Self {
        LMap {
            map: HashMap::new(),
        }
    }

    /** Add a key/value pair to an existing [`LMap`].

    # Example:
    ```
    use canrun::lmap::LMap;

    let mut map: LMap<i32, i32> = LMap::new();
    map.insert(1, 2);
    ```
    */
    pub fn insert<Ki, Vi>(&mut self, key: Ki, value: Vi)
    where
        Ki: Into<Value<K>>,
        Vi: Into<Value<V>>,
    {
        self.map.insert(key.into(), value.into());
    }

    fn resolve_in(&self, state: State) -> Option<(State, Rc<Self>)> {
        let mut state = state;
        let mut resolved: HashMap<Value<K>, Value<V>> = HashMap::new();
        for (key, value) in self.map.iter() {
            let resolved_key = state.resolve(key).clone();
            let resolved_value = state.resolve(value).clone();
            let existing = resolved.insert(resolved_key, resolved_value);
            if let Some(existing_value) = existing {
                // A variable key could end up being the same as an already
                // resolved one. They're allowed to merge IF the values unify.
                state = state.unify(value, &existing_value)?;
            }
        }
        Some((state, Rc::new(LMap { map: resolved })))
    }
}

impl<K: Unify + Eq + Hash + Debug, V: Unify + Debug> Unify for LMap<K, V> {
    fn unify(state: State, a: Rc<Self>, b: Rc<Self>) -> Option<State> {
        let (state, a) = a.resolve_in(state)?;
        let (state, b) = b.resolve_in(state)?;
        let state = unify_entries(state, a.clone(), b.clone())?;
        let state = unify_entries(state, b, a)?;
        Some(state)
    }
}

fn unify_entries<K: Unify + Eq + Hash + Debug, V: Unify + Debug>(
    mut state: State,
    a: Rc<LMap<K, V>>,
    b: Rc<LMap<K, V>>,
) -> Option<State> {
    for (a_key, a_value) in a.map.iter() {
        // In the best case, all of the keys in `a` exist in both maps
        if let Some(b_value) = b.map.get(a_key) {
            // So we can unify directly and continue or bail
            state = state.unify(a_value, b_value)?;
        } else {
            // Otherwise, we need to consider every possible match, which means
            // forking. The bad news is that this could blow up to a lot of
            // alternates if the map is large. The good news is that even if we
            // queue up a fork, any other matching keys that fail to unify will
            // abort the whole state.
            state = state.fork(LMapFork {
                a_key: a_key.clone(),
                a_value: a_value.clone(),
                b_map: b.clone(),
            })?;
        }
    }
    Some(state)
}

#[derive(Debug)]
struct LMapFork<K: Unify + Eq + Hash + Debug, V: Unify + Debug> {
    a_key: Value<K>,
    a_value: Value<V>,
    b_map: Rc<LMap<K, V>>,
}

impl<K: Unify + Eq + Hash + Debug, V: Unify + Debug> Fork for LMapFork<K, V> {
    fn fork(&self, state: &State) -> StateIter {
        let a_key = self.a_key.clone();
        let a_value = self.a_value.clone();
        let b_map = self.b_map.map.clone();
        let state = state.clone();
        Box::new(b_map.into_iter().filter_map(move |(b_key, b_value)| {
            state
                .clone()
                .unify(&a_key, &b_key)?
                .unify(&a_value, &b_value)
        }))
    }
}

impl<Kv, Kr, Vv, Vr> Reify for LMap<Kv, Vv>
where
    Kv: Unify + Eq + Hash + Reify<Reified = Kr>,
    Kr: Eq + Hash,
    Vv: Unify + Reify<Reified = Vr>,
{
    type Reified = HashMap<Kr, Vr>;
    fn reify_in(&self, state: &ReadyState) -> Option<Self::Reified> {
        let LMap { map } = self;
        let init = HashMap::with_capacity(map.len());
        map.iter().try_fold(init, |mut map, (k, v)| {
            let key = state.reify(k.clone())?;
            let value = state.reify(v.clone())?;
            map.insert(key, value);
            Some(map)
        })
    }
}

/// Create an [`LMap`](crate::lmap::LMap) with automatic key/value `Into<Value<T>>`
/// wrapping.
///
/// The primary benefit is that it allows freely mixing resolved values and
/// [`LVar`s](crate::LVar).
///
/// # Example:
/// ```
/// use canrun::LVar;
/// use canrun::lmap::{lmap, LMap};
///
/// let x = LVar::new();
/// let map = lmap!{x => 1, 2 => 3};
/// ```
#[macro_export]
macro_rules! lmap {
    ($($key:expr => $value:expr),*) => {
        {
            let mut map = $crate::lmap::LMap::new();
            $(map.insert($key, $value);)*
            map
        }
    };
}

#[doc(inline)]
pub use lmap;

use crate::{Fork, ReadyState, Reify, State, StateIter, Unify, Value};

#[cfg(test)]
mod tests {

    use crate::{goal_vec, unify, Query};
    use crate::{LVar, StateIterator};

    macro_rules! hash_map {
        ($($key:expr => $value:expr),*) => {
            {
                let mut map = std::collections::HashMap::new();
                $(map.insert($key, $value);)*
                map
            }
        };
    }

    #[test]
    fn succeeds_with_identical() {
        let goal = unify(lmap! {1 => 2}, lmap! {1 => 2});
        assert_eq!(goal.into_states().count(), 1);
    }

    #[test]
    fn fails_with_different() {
        let goal = unify(lmap! {1 => 2}, lmap! {1 => 2});
        assert_eq!(goal.into_states().count(), 1);
    }

    #[test]
    fn succeeds_with_variable_value() {
        let x = LVar::new();
        let goal = unify(lmap! {1 => 2}, lmap! {1 => x});
        let results: Vec<_> = goal.query(x).collect();
        assert_eq!(results, vec![2]);
    }

    #[test]
    fn succeeds_with_variable_key() {
        let x = LVar::new();
        let goal = unify(lmap! {1 => 2}, lmap! {x => 2});
        let results: Vec<_> = goal.query(x).collect();
        assert_eq!(results, vec![1]);
    }

    #[test]
    fn succeeds_with_variable_key_and_value() {
        let x = LVar::new();
        let y = LVar::new();
        let goal = unify(lmap! {1 => 2}, lmap! {x => y});
        let results: Vec<_> = goal.query((x, y)).collect();
        assert_eq!(results, vec![(1, 2)]);
    }

    #[test]
    fn succeeds_with_crisscrossed_variable_key_and_value() {
        let x = LVar::new();
        let y = LVar::new();
        let goal = unify(lmap! {1 => y}, lmap! {x => 2});
        let results: Vec<_> = goal.query((x, y)).collect();
        assert_eq!(results, vec![(1, 2)]);
    }

    #[test]
    fn succeeds_with_stress_test() {
        let m = LVar::new();
        let w = LVar::new();
        let x = LVar::new();
        let y = LVar::new();
        let z = LVar::new();

        let goals = goal_vec![
            unify(&m, lmap! {1 => x, 2 => w, y => x, 4 => x}),
            unify(&m, lmap! {w => 2, x => 1, 3 => x, z => x}),
        ];
        goals.assert_permutations_resolve_to(
            (m, w, x, y, z),
            vec![
                (hash_map!(1 => 2, 2 => 1, 3 => 2, 4 => 2), 1, 2, 3, 4),
                (hash_map!(2 => 2, 1 => 1, 3 => 1, 4 => 1), 2, 1, 3, 4),
            ],
        );
    }

    #[test]
    fn mergeable_keys() {
        let m = LVar::new();
        let x = LVar::new();

        let goals = goal_vec![unify(&m, lmap!(x => 1, 1 => 1)), unify(&m, lmap!(1 => 1)),];
        goals.assert_permutations_resolve_to((m, x), vec![(hash_map!(1 => 1), 1)]);
    }

    #[test]
    fn unmergeable_keys() {
        let m = LVar::new();
        let x = LVar::new();

        let goals = goal_vec![unify(&m, lmap!(x => 1, 1 => 2)), unify(&m, lmap!(1 => 2)),];
        goals.assert_permutations_resolve_to((m, x), vec![]);
    }

    #[test]
    fn debug_impl() {
        let m = LVar::new();
        let x = LVar::new();

        let goal = unify(m, lmap!(x => 1, 1 => 2));
        assert_ne!(format!("{goal:?}"), "")
    }
}
