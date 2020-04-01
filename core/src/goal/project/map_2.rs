use super::Goal;
use super::Project;
use crate::domain::{Domain, DomainType, UnifyIn};
use crate::state::State;
use crate::state::Watch;
use crate::value::{
    IntoVal, Val,
    Val::{Resolved, Var},
};
use std::fmt;
use std::rc::Rc;

pub fn map_2<'a, A, AV, B, BV, C, CV, D, ABtoC, ACtoB, BCtoA>(
    a: AV,
    b: BV,
    c: CV,
    ab_to_c: ABtoC,
    ac_to_b: ACtoB,
    bc_to_a: BCtoA,
) -> Goal<'a, D>
where
    A: UnifyIn<'a, D> + 'a,
    AV: IntoVal<A>,
    B: UnifyIn<'a, D> + 'a,
    BV: IntoVal<B>,
    C: UnifyIn<'a, D> + 'a,
    CV: IntoVal<C>,
    D: Domain<'a> + DomainType<'a, A> + DomainType<'a, B> + DomainType<'a, C>,
    ABtoC: Fn(&A, &B) -> C + 'a,
    ACtoB: Fn(&A, &C) -> B + 'a,
    BCtoA: Fn(&B, &C) -> A + 'a,
{
    Goal::Project(Rc::new(Map2 {
        a: a.into_val(),
        b: b.into_val(),
        c: c.into_val(),
        ab_to_c: Rc::new(ab_to_c),
        ac_to_b: Rc::new(ac_to_b),
        bc_to_a: Rc::new(bc_to_a),
    }))
}

pub struct Map2<'a, A, B, C> {
    a: Val<A>,
    b: Val<B>,
    c: Val<C>,
    ab_to_c: Rc<dyn Fn(&A, &B) -> C + 'a>,
    ac_to_b: Rc<dyn Fn(&A, &C) -> B + 'a>,
    bc_to_a: Rc<dyn Fn(&B, &C) -> A + 'a>,
}

impl<'a, A, B, C> fmt::Debug for Map2<'a, A, B, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Map2 ??")
    }
}

impl<'a, A, B, C, Dom> Project<'a, Dom> for Map2<'a, A, B, C>
where
    A: UnifyIn<'a, Dom>,
    B: UnifyIn<'a, Dom>,
    C: UnifyIn<'a, Dom>,
    Dom: Domain<'a> + DomainType<'a, A> + DomainType<'a, B> + DomainType<'a, C>,
{
    fn attempt<'r>(&'r self, state: State<'a, Dom>) -> Watch<State<'a, Dom>> {
        let a = state.resolve_val(&self.a).clone();
        let b = state.resolve_val(&self.b).clone();
        let c = state.resolve_val(&self.c).clone();
        match (a, b, c) {
            (Resolved(a), Resolved(b), c) => {
                let f = &self.ab_to_c;
                Watch::done(state.unify(f(&*a, &*b), c))
            }
            (Resolved(a), b, Resolved(c)) => {
                let f = &self.ac_to_b;
                Watch::done(state.unify(f(&*a, &*c), b))
            }
            (a, Resolved(b), Resolved(c)) => {
                let f = &self.bc_to_a;
                Watch::done(state.unify(f(&*b, &*c), a))
            }
            (Var(a), Var(b), _) => Watch::watch(state, a).and(b),
            (Var(a), _, Var(c)) => Watch::watch(state, a).and(c),
            (_, Var(b), Var(c)) => Watch::watch(state, b).and(c),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::map_2;
    use crate::goal::unify::unify;
    use crate::goal::Goal;
    use crate::tests::domains::Numbers;
    use crate::util;
    use crate::value::var;

    #[test]
    fn succeeds() {
        let x = var();
        let y = var();
        let z = var();
        let goals: Vec<Goal<Numbers>> = vec![
            unify(1, x),
            unify(2, y),
            unify(3, z),
            map_2(x, y, z, |x, y| x + y, |x, z| z - x, |y, z| z - y),
        ];
        util::all_permutations_resolve_to(goals, (x, y, z), vec![(1, 2, 3)]);
    }
}
