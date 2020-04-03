use canrun::domain::{Domain, DomainType, IntoDomainVal};
use canrun::goal::{any, project::Project, unify, Goal};
use canrun::state::{State, Watch};
use canrun::unify::Unify;
use canrun::value::{IntoVal, Val};
use std::fmt;
use std::iter::repeat;
use std::rc::Rc;

pub fn member<'a, I, IV, CV, D>(item: IV, collection: CV) -> Goal<'a, D>
where
    I: 'a,
    IV: IntoVal<I>,
    CV: IntoVal<Vec<Val<I>>>,
    D: Domain<'a> + DomainType<'a, I> + DomainType<'a, Vec<Val<I>>> + IntoDomainVal<'a, I>,
    State<'a, D>: Unify<'a, I> + Unify<'a, Vec<Val<I>>>,
{
    Goal::Project(Rc::new(Member {
        item: item.into_val(),
        collection: collection.into_val(),
    }))
}

struct Member<I> {
    item: Val<I>,
    collection: Val<Vec<Val<I>>>,
}

impl<'a, I, D> Project<'a, D> for Member<I>
where
    D: Domain<'a> + DomainType<'a, I> + DomainType<'a, Vec<Val<I>>> + IntoDomainVal<'a, I>,
{
    fn attempt<'r>(&'r self, state: State<'a, D>) -> Watch<State<'a, D>> {
        let collection = state.resolve_val(&self.collection).resolved();
        match collection {
            Ok(collection) => {
                let goals: Vec<_> = collection
                    .iter()
                    .zip(repeat(self.item.clone()))
                    .map(|(a, b): (&Val<I>, Val<I>)| unify(a, b) as Goal<D>)
                    .collect();
                Watch::done(any(goals).apply(state))
            }
            Err(var) => Watch::watch(state, var),
        }
    }
}

impl<I> fmt::Debug for Member<I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Member ??")
    }
}

#[cfg(test)]
mod tests {
    use super::member;
    use canrun::goal::{either, unify, Goal};
    use canrun::util;
    use canrun::value::{var, Val};
    use canrun::{domains, lvec};

    domains! {
        domain Numbers {
            i32,
            Vec<Val<i32>>
        }
    }

    #[test]
    fn basic_member() {
        let x = var::<i32>();
        let goals: Vec<Goal<Numbers>> = vec![member(x, lvec![1, 2, 3])];
        util::all_permutations_resolve_to(goals, x, vec![1, 2, 3]);
    }

    #[test]
    fn member_with_conditions() {
        let x = var();
        let goals: Vec<Goal<Numbers>> = vec![unify(x, 2), member(x, lvec![1, 2, 3])];
        util::all_permutations_resolve_to(goals, x, vec![2]);
    }

    #[test]
    fn unify_two_contains_1() {
        let x = var();
        let list = lvec![1, 2, 3];
        let goals: Vec<Goal<Numbers>> = vec![member(1, x), member(1, x), unify(x, list.clone())];
        util::all_permutations_resolve_to(goals, x, vec![list]);
    }

    #[test]
    fn unify_two_contains_2() {
        let x = var();
        let list = lvec![1, 2, 3];
        let goals: Vec<Goal<Numbers>> = vec![member(1, x), member(2, x), unify(x, list.clone())];
        util::all_permutations_resolve_to(goals, x, vec![list]);
    }

    #[test]
    fn unify_two_contains_3() {
        let x = var();
        let list = lvec![1, 2, 3];
        let goals: Vec<Goal<Numbers>> = vec![
            either(member(1, x), member(4, x)),
            member(2, x),
            unify(x, list.clone()),
        ];
        util::all_permutations_resolve_to(goals, x, vec![list]);
    }

    #[test]
    fn unify_two_contains_4() {
        let x = var();
        let list = lvec![1, 2, 3];
        let goals: Vec<Goal<Numbers>> = vec![member(1, x), member(4, x), unify(x, list.clone())];

        util::all_permutations_resolve_to(goals, x, vec![]);
    }
}
