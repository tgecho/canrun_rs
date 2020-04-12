#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_macros)]
#![allow(unused_imports)]

use canrun::{DomainType, IntoVal, LVar, State, UnifyIn, Val};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::Hash;
use std::iter::repeat;
use std::rc::Rc;

struct LMap<K, V> {
    values: HashMap<Val<K>, Val<V>>,
}

impl<K: Eq + Hash, V> LMap<K, V> {
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

    pub fn resolve_in<'a, D>(&self, state: &State<'a, D>) -> Option<Self>
    where
        D: DomainType<'a, K> + DomainType<'a, V>,
    {
        let values: HashMap<Val<K>, Val<V>> = self
            .values
            .iter()
            .map(|(key, value)| {
                let key = state.resolve_val(key).clone();
                let value = state.resolve_val(value).clone();
                (key, value)
            })
            .collect();
        if values.len() == self.values.len() {
            Some(LMap { values })
        } else {
            // If the lengths changed, then one of the keys was a var that
            // resolved to a match with one of the other keys. In theory this
            // could be ok if the values can unify, but we're going to play it
            // safe until we add support for checking.
            // When we do that, be sure to add a post resolve .len() check in the unify fn.
            None
        }
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

macro_rules! lmap {
    ($($key:expr => $value:expr),*) => {
        {
            let mut map = LMap::new();
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
    use canrun::{unify, util, val, var, Goal, IterResolved, State, UnifyIn};

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
        let w = var();
        let x = var();
        let y = var();
        let z = var();

        let goal: Goal<MapDomain> = unify(
            lmap! {1 => x, 2 => w, y => x, 4 => x},
            lmap! {w => 2, x => 1, 3 => x, z => 2},
        );
        let results: Vec<_> = goal.query((w, x, y, z)).collect();
        assert_eq!(results, vec![(1, 2, 3, 4)]);
    }
}
