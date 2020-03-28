use super::Goal;
use super::Project;
use crate::domain::{Domain, DomainType};
use crate::state::State;
use crate::state::Watch;
use crate::value::{
    IntoVal, Val,
    Val::{Resolved, Var},
};
use std::fmt;
use std::rc::Rc;

pub struct Assert2<'a, A, B> {
    a: Val<A>,
    b: Val<B>,
    f: Rc<dyn Fn(&A, &B) -> bool + 'a>,
}

pub fn assert_2<'a, A, AV, B, BV, D, F>(a: AV, b: BV, func: F) -> Goal<'a, D>
where
    A: 'a,
    AV: IntoVal<A>,
    B: 'a,
    BV: IntoVal<B>,
    D: Domain<'a> + DomainType<'a, A> + DomainType<'a, B>,
    F: Fn(&A, &B) -> bool + 'a,
{
    Goal::Project(Rc::new(Assert2 {
        a: a.into_val(),
        b: b.into_val(),
        f: Rc::new(func),
    }))
}

impl<'a, A, B, Dom> Project<'a, Dom> for Assert2<'a, A, B>
where
    Dom: Domain<'a> + DomainType<'a, A> + DomainType<'a, B>,
{
    fn attempt<'r>(&'r self, state: State<'a, Dom>) -> Watch<State<'a, Dom>> {
        let a = state.resolve_val(&self.a).clone();
        let b = state.resolve_val(&self.b).clone();
        match (a, b) {
            (Resolved(a), Resolved(b)) => Watch::done({
                let assert = self.f.clone();
                if assert(&*a, &*b) {
                    Some(state)
                } else {
                    None
                }
            }),
            (Var(var), _) => Watch::watch(state, var),
            (_, Var(var)) => Watch::watch(state, var),
        }
    }
}

impl<'a, A, B> fmt::Debug for Assert2<'a, A, B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Assert2 ??")
    }
}
