use super::{lmap, unify_entries, LMap};
use crate::{custom, project_2, DomainType, Goal, IntoVal, UnifyIn};
use std::fmt::Debug;
use std::hash::Hash;

/** Assert that [`LMap`] `a` is a subset of [`LMap`] `b`.

This means that all of the keys in `a` unify with keys in `b` AND the
corresponding values also unify. This is the opposite of [`superset`].

# Example:
```
use canrun::{var, Goal};
use canrun::lmap::{lmap, subset};
use canrun::example::Collections;

let x = var();
let goal: Goal<Collections> = subset(lmap! {x => 2}, lmap! {1 => 2, 3 => 4});
let results: Vec<_> = goal.query(x).collect();
assert_eq!(results, vec![1]);
```
*/
pub fn subset<'a, K, V, A, B, D>(a: A, b: B) -> Goal<'a, D>
where
    K: Debug + Eq + Hash + UnifyIn<'a, D> + 'a,
    V: Debug + UnifyIn<'a, D> + 'a,
    A: IntoVal<LMap<K, V>>,
    B: IntoVal<LMap<K, V>>,
    D: DomainType<'a, LMap<K, V>> + DomainType<'a, K> + DomainType<'a, V> + 'a,
{
    project_2(a, b, |a, b| {
        custom(move |state| unify_entries(state, a.clone(), b.clone()))
    })
}

/// Assert that [`LMap`] `a` is a superset of [`LMap`] `b`.
///
/// This means that all of the keys in `b` unify with keys in `a` AND the
/// corresponding values also unify. This is the opposite of [`subset`].
///
/// # Example:
/// ```
/// use canrun::{var, Goal};
/// use canrun::lmap::{lmap, superset};
/// use canrun::example::Collections;
///
/// let x = var();
/// let goal: Goal<Collections> = superset(lmap! {x => 2, 3 => 4}, lmap! {1 => 2});
/// let results: Vec<_> = goal.query(x).collect();
/// assert_eq!(results, vec![1]);
/// ```
pub fn superset<'a, K, V, A, B, D>(a: A, b: B) -> Goal<'a, D>
where
    K: Debug + Eq + Hash + UnifyIn<'a, D> + 'a,
    V: Debug + UnifyIn<'a, D> + 'a,
    A: IntoVal<LMap<K, V>>,
    B: IntoVal<LMap<K, V>>,
    D: DomainType<'a, LMap<K, V>> + DomainType<'a, K> + DomainType<'a, V> + 'a,
{
    subset(b, a)
}

/// Assert that the a given key and value combination can be found in an
/// [`LMap`]
///
/// This is essentially a single key case of [`subset`].
///
/// # Example:
/// ```
/// use canrun::{var, Goal};
/// use canrun::lmap::{lmap, get};
/// use canrun::example::Collections;
///
/// let x = var();
/// let goal: Goal<Collections> = get(1, x, lmap! {1 => 2});
/// let results: Vec<_> = goal.query(x).collect();
/// assert_eq!(results, vec![2]);
/// ```
pub fn get<'a, K, KV, V, VV, B, D>(key: KV, value: VV, b: B) -> Goal<'a, D>
where
    K: Debug + Eq + Hash + UnifyIn<'a, D> + 'a,
    KV: IntoVal<K>,
    V: Debug + UnifyIn<'a, D> + 'a,
    VV: IntoVal<V>,
    B: IntoVal<LMap<K, V>>,
    D: DomainType<'a, LMap<K, V>> + DomainType<'a, K> + DomainType<'a, V> + 'a,
{
    subset(lmap! {key => value}, b)
}

#[cfg(test)]
mod tests {
    use super::{get, subset, superset};
    use crate::example::Collections;
    use crate::lmap;
    use crate::{var, Goal, IterResolved};

    #[test]
    fn get_succeeds() {
        let x = var();
        let goal: Goal<Collections> = get(1, x, lmap! {1 => 2});
        let results: Vec<_> = goal.query(x).collect();
        assert_eq!(results, vec![2]);
    }

    #[test]
    fn subset_should_succeed_on() {
        let x = var();
        let cases = vec![
            (lmap! {1 => 2}, lmap! {1 => 2}),
            (lmap! {1 => 2}, lmap! {1 => 2, 3 => 4}),
            (lmap! {x => 2}, lmap! {1 => 2, 3 => 4}),
            (lmap! {x => 2}, lmap! {x => 2, 3 => 4}),
        ];
        for (a, b) in cases {
            let goal: Goal<Collections> = subset(&a, &b);
            assert_eq!(goal.iter_resolved().count(), 1, "case: {a:?} {b:?}");
        }
    }

    #[test]
    fn subset_should_fail_on() {
        let x = var();
        let cases = vec![
            (lmap! {1 => 2, 3 => 4}, lmap! {1 => 2}),
            (lmap! {x => 2}, lmap! {1 => 1}),
            (lmap! {x => 2}, lmap! {1 => 2, x => 4}),
        ];
        for (a, b) in cases {
            let goal: Goal<Collections> = subset(&a, &b);
            assert_eq!(goal.iter_resolved().count(), 0, "case: {a:?} {b:?}");
        }
    }

    #[test]
    fn superset_should_succeed_on() {
        let x = var();
        let cases = vec![
            (lmap! {1 => 2}, lmap! {1 => 2}),
            (lmap! {1 => 2, 3 => 4}, lmap! {1 => 2}),
            (lmap! {x => 2, 3 => 4}, lmap! {x => 2}),
        ];
        for (a, b) in cases {
            let goal: Goal<Collections> = superset(&a, &b);
            assert_eq!(goal.iter_resolved().count(), 1, "case: {a:?} {b:?}");
        }
    }

    #[test]
    fn superset_should_fail_on() {
        let x = var();
        let cases = vec![
            (lmap! {1 => 2}, lmap! {1 => 2, 3 => 4}),
            (lmap! {x => 2}, lmap! {1 => 1}),
            (lmap! {1 => 2, x => 4}, lmap! {x => 2}),
        ];
        for (a, b) in cases {
            let goal: Goal<Collections> = superset(&a, &b);
            assert_eq!(goal.iter_resolved().count(), 0, "case: {a:?} {b:?}");
        }
    }
}
