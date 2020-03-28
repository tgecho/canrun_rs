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

pub struct Assert1<'a, A> {
    a: Val<A>,
    f: Rc<dyn Fn(&A) -> bool + 'a>,
}

pub fn assert_1<'a, A, AV, D, F>(a: AV, func: F) -> Goal<'a, D>
where
    A: 'a,
    AV: IntoVal<A>,
    D: Domain<'a> + DomainType<'a, A>,
    F: Fn(&A) -> bool + 'a,
{
    Goal::Project(Rc::new(Assert1 {
        a: a.into_val(),
        f: Rc::new(func),
    }))
}

impl<'a, A, Dom> Project<'a, Dom> for Assert1<'a, A>
where
    Dom: Domain<'a> + DomainType<'a, A>,
{
    fn attempt<'r>(&'r self, state: State<'a, Dom>) -> Watch<State<'a, Dom>> {
        let a = state.resolve_val(&self.a).clone();
        match a {
            Resolved(a) => Watch::done({
                let assert = self.f.clone();
                if assert(&*a) {
                    Some(state)
                } else {
                    None
                }
            }),
            Var(var) => Watch::watch(state, var),
        }
    }
}

impl<'a, A> fmt::Debug for Assert1<'a, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Assert1 ??")
    }
}
