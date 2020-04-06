use super::Unify;
use crate::domains::DomainType;
use crate::state::State;
use crate::value::Val;
use std::rc::Rc;

impl<'a, A, B, D> Unify<'a, (Val<A>, Val<B>)> for D
where
    D: Unify<'a, A> + Unify<'a, B> + DomainType<'a, (Val<A>, Val<B>)>,
{
    fn unify_resolved(
        state: State<'a, D>,
        l: Rc<(Val<A>, Val<B>)>,
        r: Rc<(Val<A>, Val<B>)>,
    ) -> Option<State<'a, D>> {
        Some(
            state
                .unify(l.0.clone(), r.0.clone())?
                .unify(l.1.clone(), r.1.clone())?,
        )
    }
}

impl<'a, A, B, C, D> Unify<'a, (Val<A>, Val<B>, Val<C>)> for D
where
    D: Unify<'a, A> + Unify<'a, B> + Unify<'a, C> + DomainType<'a, (Val<A>, Val<B>, Val<C>)>,
{
    fn unify_resolved(
        state: State<'a, D>,
        l: Rc<(Val<A>, Val<B>, Val<C>)>,
        r: Rc<(Val<A>, Val<B>, Val<C>)>,
    ) -> Option<State<'a, D>> {
        Some(
            state
                .unify(l.0.clone(), r.0.clone())?
                .unify(l.1.clone(), r.1.clone())?
                .unify(l.2.clone(), r.2.clone())?,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate as canrun;
    use crate::goal::unify;
    use crate::goal::Goal;
    use crate::util;
    use crate::value::{val, var, Val};
    use canrun_codegen::domains;

    domains! {
        pub domain Tuples2 {
            i32,
            (Val<i32>, Val<i32>),
        }
        pub domain Tuples3 {
            i32,
            (Val<i32>, Val<i32>, Val<i32>),
        }
    }

    #[test]
    fn tuple2_succeeds() {
        let x = var();
        let goals: Vec<Goal<Tuples2>> =
            vec![unify(x, (val!(1), val!(2))), unify(x, (val!(1), val!(2)))];
        util::all_permutations_resolve_to(goals, x, vec![(val!(1), val!(2))]);
    }

    #[test]
    fn tuple2_fails() {
        let x = var();
        let goals: Vec<Goal<Tuples2>> =
            vec![unify(x, (val!(1), val!(3))), unify(x, (val!(1), val!(2)))];
        util::all_permutations_resolve_to(goals, x, vec![]);
    }

    #[test]
    fn tuple2_nested_var() {
        let x = var();
        let y = var();
        let goals: Vec<Goal<Tuples2>> =
            vec![unify(x, (val!(1), val!(y))), unify(x, (val!(1), val!(2)))];
        util::all_permutations_resolve_to(goals, y, vec![2]);
    }

    #[test]
    fn tuple3_succeeds() {
        let x = var();
        let goals: Vec<Goal<Tuples3>> = vec![
            unify(x, (val!(1), val!(2), val!(3))),
            unify(x, (val!(1), val!(2), val!(3))),
        ];
        util::all_permutations_resolve_to(goals, x, vec![(val!(1), val!(2), val!(3))]);
    }

    #[test]
    fn tuple3_fails() {
        let x = var();
        let goals: Vec<Goal<Tuples3>> = vec![
            unify(x, (val!(1), val!(2), val!(3))),
            unify(x, (val!(1), val!(2), val!(4))),
        ];
        util::all_permutations_resolve_to(goals, x, vec![]);
    }

    #[test]
    fn tuple3_nested_var() {
        let x = var();
        let y = var();
        let goals: Vec<Goal<Tuples3>> = vec![
            unify(x, (val!(1), val!(y), val!(3))),
            unify(x, (val!(1), val!(2), val!(3))),
        ];
        util::all_permutations_resolve_to(goals, y, vec![2]);
    }
}
