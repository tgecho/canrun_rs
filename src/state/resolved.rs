use super::{State, WatchFns};
use crate::domain::{Domain, DomainType};
use crate::value::{LVar, Val};
use std::rc::Rc;

#[derive(Clone)]
pub struct ResolvedState<'a, D: Domain<'a> + 'a> {
    pub(super) domain: D,
    pub(super) watches: WatchFns<'a, D>,
}

impl<'a, D: Domain<'a> + 'a> ResolvedState<'a, D> {
    pub fn get_rc<T>(&self, var: &LVar<T>) -> Option<Rc<T>>
    where
        D: DomainType<'a, T>,
    {
        let val = self.domain.values_as_ref().get(var)?;
        match val {
            Val::Var(var) => self.get_rc(var),
            Val::Resolved(resolved) => Some(resolved.clone()),
        }
    }

    pub fn get<T>(&self, var: &LVar<T>) -> Option<T>
    where
        T: Clone,
        D: DomainType<'a, T>,
    {
        self.get_rc(var).map(|rc| (*rc).clone())
    }

    pub fn reopen(self) -> State<'a, D> {
        State {
            domain: self.domain,
            watches: self.watches,
            forks: im::Vector::new(),
        }
    }
}
