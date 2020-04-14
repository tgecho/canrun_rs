//! A map-like data structure with [`LVar`](canrun::value::LVar) keys and values.
use canrun::state::{Fork, StateIter};
use canrun::{DomainType, IntoVal, ReifyIn, ResolvedState, State, UnifyIn, Val};
use std::collections::HashMap;
use std::fmt;
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;

mod compare;

pub use compare::{is_subset, is_superset};

/// A map-like data structure with [`LVar`](canrun::value::LVar) keys and values.
#[derive(Debug, Clone)]
pub struct LMap<K: Eq + Hash + Debug, V: Debug> {
    values: HashMap<Val<K>, Val<V>>,
}

impl<K: Eq + Hash + Debug, V: Debug> LMap<K, V> {
    /// Create a new [`LMap`] value.
    ///
    /// You may also be interested in the [`lmap!`] macro.
    ///
    /// # Example:
    /// ```
    /// use canrun_collections::lmap::LMap;
    ///
    /// let map: LMap<i32, i32> = LMap::new();
    /// ```
    pub fn new() -> Self {
        LMap {
            values: HashMap::new(),
        }
    }

    /// Add a key/value pair to an existing [`LMap`].
    ///
    /// # Example:
    /// ```
    /// use canrun_collections::lmap::LMap;
    ///
    /// let mut map: LMap<i32, i32> = LMap::new();
    /// map.insert(1, 2);
    /// ```
    pub fn insert<Ki, Vi>(&mut self, key: Ki, value: Vi)
    where
        Ki: IntoVal<K>,
        Vi: IntoVal<V>,
    {
        self.values.insert(key.into_val(), value.into_val());
    }

    fn resolve_in<'a, D>(&self, state: State<'a, D>) -> Option<(State<'a, D>, Self)>
    where
        V: UnifyIn<'a, D>,
        K: UnifyIn<'a, D>,
        D: DomainType<'a, K> + DomainType<'a, V>,
    {
        let mut state = state;
        let mut resolved: HashMap<Val<K>, Val<V>> = HashMap::new();
        for (key, value) in self.values.iter() {
            let resolved_key = state.resolve_val(&key).clone();
            let resolved_value = state.resolve_val(&value).clone();
            let existing = resolved.insert(resolved_key, resolved_value);
            if let Some(existing_value) = existing {
                // A variable key could end up being the same as an already
                // resolved one. They're allowed to merge IF the values unify.
                state = state.unify(&value, &existing_value)?;
            }
        }
        Some((state, LMap { values: resolved }))
    }
}

impl<'a, K, V, D> UnifyIn<'a, D> for LMap<K, V>
where
    K: UnifyIn<'a, D> + Eq + Hash + fmt::Debug + 'a,
    V: UnifyIn<'a, D> + fmt::Debug + 'a,
    D: DomainType<'a, K> + DomainType<'a, V> + DomainType<'a, Self>,
{
    fn unify_resolved(state: State<'a, D>, a: Rc<Self>, b: Rc<Self>) -> Option<State<'a, D>> {
        let (state, a) = a.resolve_in(state)?;
        let (state, b) = b.resolve_in(state)?;
        let state = unify_entries(state, &a.values, &b.values)?;
        let state = unify_entries(state, &b.values, &a.values)?;
        Some(state)
    }
}

fn unify_entries<'a, K, V, D>(
    mut state: State<'a, D>,
    a_entries: &HashMap<Val<K>, Val<V>>,
    b_entries: &HashMap<Val<K>, Val<V>>,
) -> Option<State<'a, D>>
where
    K: UnifyIn<'a, D> + Eq + Hash + fmt::Debug + 'a,
    V: UnifyIn<'a, D> + fmt::Debug + 'a,
    D: DomainType<'a, K> + DomainType<'a, V>,
{
    for (a_key, a_value) in a_entries.iter() {
        // In the best case, all of the keys in `a` exist in both maps
        if let Some(b_value) = b_entries.get(a_key) {
            // So we can unify directly and continue or bail
            state = state.unify(a_value, b_value)?;
        } else {
            // Otherwise, we need to consider every possible match, which means
            // forking. The bad news is that this could blow up to a lot of
            // alternates if the map is large The good news is that even if we
            // queue up a fork, any other matching keys that fail to unify will
            // abort the whole state.
            //
            // TODO: Either figure out a way to not do so much ugly cloning
            // (especially b_values) or make sure the cost is not bad and/or
            // mitigated with something like im::HashMap. Also look in LMapFork.
            // Measure!
            state = state.fork(Rc::new(LMapFork {
                a_key: a_key.clone(),
                a_value: a_value.clone(),
                b_values: b_entries.clone(),
            }))?;
        }
    }
    Some(state)
}

#[derive(Debug)]
struct LMapFork<K: Eq + Hash + Debug, V: Debug> {
    a_key: Val<K>,
    a_value: Val<V>,
    b_values: HashMap<Val<K>, Val<V>>,
}

impl<'a, K: Eq + Hash + Debug, V: Debug, D> Fork<'a, D> for LMapFork<K, V>
where
    K: UnifyIn<'a, D> + 'a,
    V: UnifyIn<'a, D> + 'a,
    D: DomainType<'a, K> + DomainType<'a, V>,
{
    fn fork(&self, state: State<'a, D>) -> StateIter<'a, D> {
        let a_key = self.a_key.clone();
        let a_value = self.a_value.clone();
        let b_values = self.b_values.clone();
        Box::new(b_values.into_iter().filter_map(move |(b_key, b_value)| {
            state
                .clone()
                .unify(&a_key, &b_key)?
                .unify(&a_value, &b_value)
        }))
    }
}

impl<'a, D, Kv: Eq + Hash + Debug, Kr, Vv: Debug, Vr> ReifyIn<'a, D> for LMap<Kv, Vv>
where
    D: DomainType<'a, Kv> + DomainType<'a, Vv> + 'a,
    Kv: ReifyIn<'a, D, Reified = Kr>,
    Kr: Eq + Hash,
    Vv: ReifyIn<'a, D, Reified = Vr>,
{
    type Reified = HashMap<Kr, Vr>;
    fn reify_in(&self, state: &ResolvedState<D>) -> Option<Self::Reified> {
        let LMap { values } = self;
        let init = HashMap::with_capacity(values.len());
        values.iter().try_fold(init, |mut map, (k, v)| {
            let key = state.reify(k)?;
            let value = state.reify(v)?;
            map.insert(key, value);
            Some(map)
        })
    }
}

/// Create an [`LMap`](crate::lmap::LMap).
///
/// # Example:
/// ```
/// use canrun::var;
/// use canrun_collections::lmap::{lmap, LMap};
///
/// let x = var();
/// let map: LMap<i32, i32> = lmap!(x => 1, 2 => 3);
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

#[cfg(test)]
mod tests {
    use crate::example::LMapI32;
    use crate::lmap;
    use canrun::{unify, util, var, Goal, IterResolved};

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
        let goal: Goal<LMapI32> = unify(lmap! {1 => 2}, lmap! {1 => 2});
        assert_eq!(goal.iter_resolved().count(), 1);
    }

    #[test]
    fn fails_with_different() {
        let goal: Goal<LMapI32> = unify(lmap! {1 => 2}, lmap! {1 => 2});
        assert_eq!(goal.iter_resolved().count(), 1);
    }

    #[test]
    fn succeeds_with_variable_value() {
        let x = var();
        let goal: Goal<LMapI32> = unify(lmap! {1 => 2}, lmap! {1 => x});
        let results: Vec<_> = goal.query(x).collect();
        assert_eq!(results, vec![2]);
    }

    #[test]
    fn succeeds_with_variable_key() {
        let x = var();
        let goal: Goal<LMapI32> = unify(lmap! {1 => 2}, lmap! {x => 2});
        let results: Vec<_> = goal.query(x).collect();
        assert_eq!(results, vec![1]);
    }

    #[test]
    fn succeeds_with_variable_key_and_value() {
        let x = var();
        let y = var();
        let goal: Goal<LMapI32> = unify(lmap! {1 => 2}, lmap! {x => y});
        let results: Vec<_> = goal.query((x, y)).collect();
        assert_eq!(results, vec![(1, 2)]);
    }

    #[test]
    fn succeeds_with_crisscrossed_variable_key_and_value() {
        let x = var();
        let y = var();
        let goal: Goal<LMapI32> = unify(lmap! {1 => y}, lmap! {x => 2});
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

        let goals: Vec<Goal<LMapI32>> = vec![
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

    #[test]
    fn mergeable_keys() {
        let m = var();
        let x = var();

        let goals: Vec<Goal<LMapI32>> =
            vec![unify(m, lmap!(x => 1, 1 => 1)), unify(m, lmap!(1 => 1))];
        util::assert_permutations_resolve_to(goals, (m, x), vec![(hash_map!(1 => 1), 1)]);
    }

    #[test]
    fn unmergeable_keys() {
        let m = var();
        let x = var();

        let goals: Vec<Goal<LMapI32>> =
            vec![unify(m, lmap!(x => 1, 1 => 2)), unify(m, lmap!(1 => 2))];
        util::assert_permutations_resolve_to(goals, (m, x), vec![]);
    }
}
