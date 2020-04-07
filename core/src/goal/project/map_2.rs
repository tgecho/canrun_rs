use super::Goal;
use super::Project;
use crate::state::State;
use crate::state::Watch;
use crate::unify::Unify;
use crate::value::{
    IntoVal, Val,
    Val::{Resolved, Var},
};
use std::fmt;
use std::rc::Rc;

/// Create a [projection goal](super) that allows deriving one resolved value
/// from the other two.
///
/// Functions must be provided to derive from any combination of two values. Whichever two are
/// resolved first will be used to derive the other.
///
/// ```
/// use canrun::{Goal, all, unify, var, map_2};
/// use canrun::domains::example::I32;
///
/// let (x, y, z) = (var(), var(), var());
/// let goal: Goal<I32> = all![
///     unify(1, x),
///     unify(2, y),
///     map_2(x, y, z, |x, y| x + y, |x, z| z - x, |y, z| z - y),
/// ];
/// let result: Vec<_> = goal.query(z).collect();
/// assert_eq!(result, vec![3])
/// ```
pub fn map_2<'a, A: 'a, AV, B: 'a, BV, C: 'a, CV, D, ABtoC, ACtoB, BCtoA>(
    a: AV,
    b: BV,
    c: CV,
    ab_to_c: ABtoC,
    ac_to_b: ACtoB,
    bc_to_a: BCtoA,
) -> Goal<'a, D>
where
    AV: IntoVal<A>,
    BV: IntoVal<B>,
    CV: IntoVal<C>,
    D: Unify<'a, A> + Unify<'a, B> + Unify<'a, C>,
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
    Dom: Unify<'a, A> + Unify<'a, B> + Unify<'a, C> + 'a,
{
    fn attempt<'r>(&'r self, state: State<'a, Dom>) -> Watch<State<'a, Dom>> {
        let a = state.resolve_val(&self.a).clone();
        let b = state.resolve_val(&self.b).clone();
        let c = state.resolve_val(&self.c).clone();
        match (a, b, c) {
            (Resolved(a), Resolved(b), c) => {
                let f = &self.ab_to_c;
                Watch::done(state.unify(&f(&*a, &*b).into_val(), &c))
            }
            (Resolved(a), b, Resolved(c)) => {
                let f = &self.ac_to_b;
                Watch::done(state.unify(&f(&*a, &*c).into_val(), &b))
            }
            (a, Resolved(b), Resolved(c)) => {
                let f = &self.bc_to_a;
                Watch::done(state.unify(&f(&*b, &*c).into_val(), &a))
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
    use crate::domains::example::I32;
    use crate::goal::unify::unify;
    use crate::goal::Goal;
    use crate::util;
    use crate::value::var;

    #[test]
    fn succeeds() {
        let x = var();
        let y = var();
        let z = var();
        let goals: Vec<Goal<I32>> = vec![
            unify(1, x),
            unify(2, y),
            unify(3, z),
            map_2(x, y, z, |x, y| x + y, |x, z| z - x, |y, z| z - y),
        ];
        util::assert_permutations_resolve_to(goals, (x, y, z), vec![(1, 2, 3)]);
    }
}
