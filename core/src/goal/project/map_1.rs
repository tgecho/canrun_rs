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

pub fn map_1<'a, A, AV, B, BV, D, A_, _B>(a: AV, b: BV, a_: A_, _b: _B) -> Goal<'a, D>
where
    A: UnifyIn<'a, D> + 'a,
    AV: IntoVal<A>,
    B: UnifyIn<'a, D> + 'a,
    BV: IntoVal<B>,
    D: Domain<'a> + DomainType<'a, A> + DomainType<'a, B>,
    A_: Fn(&A) -> B + 'a,
    _B: Fn(&B) -> A + 'a,
{
    Goal::Project(Rc::new(Map1 {
        a: a.into_val(),
        b: b.into_val(),
        a_: Rc::new(a_),
        _b: Rc::new(_b),
    }))
}

pub struct Map1<'a, A, B> {
    a: Val<A>,
    b: Val<B>,
    a_: Rc<dyn Fn(&A) -> B + 'a>,
    _b: Rc<dyn Fn(&B) -> A + 'a>,
}

impl<'a, A, B> fmt::Debug for Map1<'a, A, B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Map1 ??")
    }
}

impl<'a, A, B, Dom> Project<'a, Dom> for Map1<'a, A, B>
where
    A: UnifyIn<'a, Dom>,
    B: UnifyIn<'a, Dom>,
    Dom: Domain<'a> + DomainType<'a, A> + DomainType<'a, B>,
{
    fn attempt<'r>(&'r self, state: State<'a, Dom>) -> Watch<State<'a, Dom>> {
        let a = state.resolve_val(&self.a).clone();
        let b = state.resolve_val(&self.b).clone();
        match (a, b) {
            (Resolved(a), b) => {
                let f = &self.a_;
                Watch::done(state.unify(f(&*a), b))
            }
            (a, Resolved(b)) => {
                let f = &self._b;
                Watch::done(state.unify(f(&*b), a))
            }
            (Var(a), Var(b)) => Watch::watch(state, a).and(b),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::map_1;
    use crate::domain::one::OfOne;
    use crate::goal::unify::unify;
    use crate::goal::Goal;
    use crate::tests::util;
    use crate::value::var;

    #[test]
    fn succeeds() {
        let x = var();
        let y = var();
        let goals: Vec<Goal<OfOne<i32>>> =
            vec![unify(1, x), unify(2, y), map_1(x, y, |x| x + 1, |y| y - 1)];
        util::all_permutations_resolve_to(goals, (x, y), vec![(1, 2)]);
    }
}
