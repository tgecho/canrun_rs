#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_macros)]
#![allow(unused_imports)]

use canrun::{DomainType, IntoVal, LVar, ReifyVal, ResolvedState, State, UnifyIn, Val};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fmt::Debug;
use std::hash::Hash;
use std::iter::repeat;
use std::rc::Rc;

#[derive(Debug)]
struct LMap<K: Debug, V: Debug> {
    values: HashMap<Val<K>, Val<V>>,
}

impl<K: Eq + Hash + Debug, V: Debug> LMap<K, V> {
    pub fn new() -> Self {
        LMap {
            values: HashMap::new(),
        }
    }

    pub fn insert<Ki, Vi>(&mut self, key: Ki, value: Vi)
    where
        Ki: IntoVal<K>,
        Vi: IntoVal<V>,
    {
        self.values.insert(key.into_val(), value.into_val());
    }

    pub fn get<Ki>(&self, key: Ki) -> Option<&Val<V>>
    where
        Ki: IntoVal<K>,
    {
        self.values.get(&key.into_val())
    }
}

impl<'a, K: Eq + Hash + 'a + fmt::Debug, V: 'a + fmt::Debug, D> UnifyIn<'a, D> for LMap<K, V>
where
    K: UnifyIn<'a, D>,
    V: UnifyIn<'a, D>,
    D: DomainType<'a, K> + DomainType<'a, V> + DomainType<'a, Self>,
{
    fn unify_resolved(
        state: canrun::State<'a, D>,
        a: std::rc::Rc<Self>,
        b: std::rc::Rc<Self>,
    ) -> Option<canrun::State<'a, D>> {
        if a.values.len() != b.values.len() {
            return None;
        }

        let mut state = state;

        // let a = a.resolve_in(&state)?;
        // let b = b.resolve_in(&state)?;

        // let mut a_vars: Vec<(&Val<K>, &Val<V>)> = Vec::new();
        // let mut unified = HashSet::new();

        // for (a_key, a_value) in a.values.iter() {
        //     if let Some(b_value) = b.values.get(a_key) {
        //         state = state.unify(a_value, b_value)?;
        //     } else if a_key.is_var() {
        //         a_vars.push((a_key, a_value));
        //     }
        // } // repeat this for b?
        // if !a_vars.is_empty() { // also do this for b?

        state = state.fork(Rc::new(move |state: State<'a, D>| {
            let a_perms = a.values.clone().into_iter().permutations(a.values.len());
            let values = repeat(b.values.clone()).zip(a_perms);
            let iter = repeat(state)
                .zip(values)
                .filter_map(|(state, (b_values, a_values))| {
                    b_values
                        .into_iter()
                        .zip(a_values.into_iter())
                        .try_fold(state, |s, ((a_k, a_v), (b_k, b_v))| {
                            s.unify(&a_k, &b_k)?.unify(&a_v, &b_v)
                        })
                });
            Box::new(iter)
        }))?;

        Some(state)
    }
}

impl<'a, D, Kv: Debug, Kr, Vv: Debug, Vr> ReifyVal<'a, D> for LMap<Kv, Vv>
where
    D: DomainType<'a, Kv> + DomainType<'a, Vv> + 'a,
    Kv: ReifyVal<'a, D, Reified = Kr>,
    Kr: Eq + Hash,
    Vv: ReifyVal<'a, D, Reified = Vr>,
{
    type Reified = HashMap<Kr, Vr>;
    fn reify_in(&self, state: &ResolvedState<D>) -> Option<Self::Reified> {
        let LMap { values } = self;
        let init = HashMap::with_capacity(values.len());
        values.iter().try_fold(init, |mut map, (k, v)| {
            let key = state.reify_val(k)?;
            let value = state.reify_val(v)?;
            map.insert(key, value);
            Some(map)
        })
    }
}

macro_rules! lmap {
    ($($key:expr => $value:expr),*) => {
        {
            let mut map = canrun_collections::lmap::LMap::new();
            $(map.insert($key, $value);)*
            map
        }
    };
}
macro_rules! hash_map {
    ($($key:expr => $value:expr),*) => {
        {
            let mut map = std::collections::HashMap::new();
            $(map.insert($key, $value);)*
            map
        }
    };
}

impl<'a, K, V> fmt::Debug for LMap<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LMap ??")
    }
}

#[cfg(test)]
mod tests {
    use super::LMap;
    use crate as canrun_collections;
    use canrun::{all, unify, util, val, var, Goal, IterResolved, State, UnifyIn};

    canrun::domain! {
        MapDomain {
            i32,
            LMap<i32, i32>,
        }
    }

    #[test]
    fn succeeds_with_identical() {
        let goal: Goal<MapDomain> = unify(lmap! {1 => 2}, lmap! {1 => 2});
        assert_eq!(goal.iter_resolved().count(), 1);
    }

    #[test]
    fn fails_with_different() {
        let goal: Goal<MapDomain> = unify(lmap! {1 => 2}, lmap! {1 => 2});
        assert_eq!(goal.iter_resolved().count(), 1);
    }

    #[test]
    fn succeeds_with_variable_value() {
        let x = var();
        let goal: Goal<MapDomain> = unify(lmap! {1 => 2}, lmap! {1 => x});
        let results: Vec<_> = goal.query(x).collect();
        assert_eq!(results, vec![2]);
    }

    #[test]
    fn succeeds_with_variable_key() {
        let x = var();
        let goal: Goal<MapDomain> = unify(lmap! {1 => 2}, lmap! {x => 2});
        let results: Vec<_> = goal.query(x).collect();
        assert_eq!(results, vec![1]);
    }

    #[test]
    fn succeeds_with_variable_key_and_value() {
        let x = var();
        let y = var();
        let goal: Goal<MapDomain> = unify(lmap! {1 => 2}, lmap! {x => y});
        let results: Vec<_> = goal.query((x, y)).collect();
        assert_eq!(results, vec![(1, 2)]);
    }

    #[test]
    fn succeeds_with_crisscrossed_variable_key_and_value() {
        let x = var();
        let y = var();
        let goal: Goal<MapDomain> = unify(lmap! {1 => y}, lmap! {x => 2});
        let results: Vec<_> = goal.query((x, y)).collect();
        assert_eq!(results, vec![(1, 2)]);
    }

    #[test]
    fn succeeds_with_stress_test() {
        let m = var();
        let w = var();
        let x = var();
        let y = var();
        let z = var();

        let goals: Vec<Goal<MapDomain>> = vec![
            unify(m, lmap! {1 => x, 2 => w, y => x, 4 => x}),
            unify(m, lmap! {w => 2, x => 1, 3 => x, z => x}),
        ];
        util::assert_permutations_resolve_to(
            goals,
            (m, w, x, y, z),
            vec![
                (hash_map!(1 => 2, 2 => 1, 3 => 2, 4 => 2), 1, 2, 3, 4),
                (hash_map!(2 => 2, 1 => 1, 3 => 1, 4 => 1), 2, 1, 3, 4),
            ],
        );
    }
}
