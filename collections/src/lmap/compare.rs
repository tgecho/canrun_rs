use super::{unify_entries, LMap};
use canrun::{custom, project_2, DomainType, Goal, IntoVal, UnifyIn};
use std::fmt::Debug;
use std::hash::Hash;

/// Assert that [`LMap`] `a` is a subset of [`LMap`] `b`.
///
/// This means that all of the keys in `a` unify with keys in `b` AND the
/// corresponding values also unify. This is the opposite of [`is_superset`].
///
/// # Example:
/// ```
/// use canrun::{var, Goal};
/// use canrun_collections::lmap::{lmap, is_subset};
/// use canrun_collections::example::LMapI32;
///
/// let x = var();
/// let goal: Goal<LMapI32> = is_subset(lmap! {x => 2}, lmap! {1 => 2, 3 => 4});
/// let results: Vec<_> = goal.query(x).collect();
/// assert_eq!(results, vec![1]);
/// ```
pub fn is_subset<'a, K, V, A, B, D>(a: A, b: B) -> Goal<'a, D>
where
    K: Debug + Eq + Hash + UnifyIn<'a, D> + 'a,
    V: Debug + UnifyIn<'a, D> + 'a,
    A: IntoVal<LMap<K, V>>,
    B: IntoVal<LMap<K, V>>,
    D: DomainType<'a, LMap<K, V>> + DomainType<'a, K> + DomainType<'a, V> + 'a,
{
    project_2(a, b, |a, b| {
        let a = a.values.clone();
        let b = b.values.clone();
        custom(move |state| unify_entries(state, &a, &b))
    })
}

/// Assert that [`LMap`] `a` is a superset of [`LMap`] `b`.
///
/// This means that all of the keys in `b` unify with keys in `a` AND the
/// corresponding values also unify. This is the opposite of [`is_subset`].
///
/// # Example:
/// ```
/// use canrun::{var, Goal};
/// use canrun_collections::lmap::{lmap, is_superset};
/// use canrun_collections::example::LMapI32;
///
/// let x = var();
/// let goal: Goal<LMapI32> = is_superset(lmap! {x => 2, 3 => 4}, lmap! {1 => 2});
/// let results: Vec<_> = goal.query(x).collect();
/// assert_eq!(results, vec![1]);
/// ```
pub fn is_superset<'a, K, V, A, B, D>(a: A, b: B) -> Goal<'a, D>
where
    K: Debug + Eq + Hash + UnifyIn<'a, D> + 'a,
    V: Debug + UnifyIn<'a, D> + 'a,
    A: IntoVal<LMap<K, V>>,
    B: IntoVal<LMap<K, V>>,
    D: DomainType<'a, LMap<K, V>> + DomainType<'a, K> + DomainType<'a, V> + 'a,
{
    is_subset(b, a)
}

#[cfg(test)]
mod tests {
    use super::{is_subset, is_superset};
    use crate::example::LMapI32;
    use crate::lmap;
    use canrun::{var, Goal, IterResolved};

    #[test]
    fn is_subset_should_succeed_on() {
        let x = var();
        let cases = vec![
            (lmap! {1 => 2}, lmap! {1 => 2}),
            (lmap! {1 => 2}, lmap! {1 => 2, 3 => 4}),
            (lmap! {x => 2}, lmap! {1 => 2, 3 => 4}),
            (lmap! {x => 2}, lmap! {x => 2, 3 => 4}),
        ];
        for (a, b) in cases {
            let goal: Goal<LMapI32> = is_subset(&a, &b);
            if goal.iter_resolved().count() != 1 {
                panic!("is_subset failed on {:?} {:?}", a, b);
            }
        }
    }

    #[test]
    fn is_subset_should_fail_on() {
        let x = var();
        let cases = vec![
            (lmap! {1 => 2, 3 => 4}, lmap! {1 => 2}),
            (lmap! {x => 2}, lmap! {1 => 1}),
            (lmap! {x => 2}, lmap! {1 => 2, x => 4}),
        ];
        for (a, b) in cases {
            let goal: Goal<LMapI32> = is_subset(&a, &b);
            if goal.iter_resolved().count() != 0 {
                panic!("is_subset erroneously succeeded on {:?} {:?}", a, b);
            }
        }
    }

    #[test]
    fn is_superset_should_succeed_on() {
        let x = var();
        let cases = vec![
            (lmap! {1 => 2}, lmap! {1 => 2}),
            (lmap! {1 => 2, 3 => 4}, lmap! {1 => 2}),
            (lmap! {x => 2, 3 => 4}, lmap! {x => 2}),
        ];
        for (a, b) in cases {
            let goal: Goal<LMapI32> = is_superset(&a, &b);
            if goal.iter_resolved().count() != 1 {
                panic!("is_superset failed on {:?} {:?}", a, b);
            }
        }
    }

    #[test]
    fn is_superset_should_fail_on() {
        let x = var();
        let cases = vec![
            (lmap! {1 => 2}, lmap! {1 => 2, 3 => 4}),
            (lmap! {x => 2}, lmap! {1 => 1}),
            (lmap! {1 => 2, x => 4}, lmap! {x => 2}),
        ];
        for (a, b) in cases {
            let goal: Goal<LMapI32> = is_superset(&a, &b);
            if goal.iter_resolved().count() != 0 {
                panic!("is_superset erroneously succeeded on {:?} {:?}", a, b);
            }
        }
    }
}
