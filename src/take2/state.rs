use crate::can::lvar::LVar;
use crate::take2::domain::{Domain, DomainType};
use crate::take2::goals::StateIter;
use crate::take2::val::{
    Val,
    Val::{Resolved, Var},
};

pub struct State<D: Domain> {
    domain: D,
}

impl<'a, D: Domain + 'a> State<D> {
    pub fn new() -> Self {
        State { domain: D::new() }
    }

    pub fn to_iter(self) -> StateIter<'a, Self> {
        Box::new(std::iter::once(self))
    }
}

pub trait Resolve<T> {
    fn resolve<'a>(&'a self, key: &'a Val<T>) -> &'a Val<T>;
}

impl<T, D: DomainType<T>> Resolve<T> for State<D> {
    fn resolve<'a>(&'a self, key: &'a Val<T>) -> &'a Val<T> {
        match key {
            Val::Var(var) => self.domain.values_as_ref().get(var).unwrap_or(key),
            value => value,
        }
    }
}

pub trait Unify<'a, T>: Assign<T> {
    fn unify(self, a: Val<T>, b: Val<T>) -> StateIter<'a, Self>;
}

impl<'a, T: PartialEq, D: DomainType<T> + 'a> Unify<'a, T> for State<D> {
    fn unify(self, a: Val<T>, b: Val<T>) -> StateIter<'a, Self> {
        match (a, b) {
            (a, b) if a == b => self.to_iter(),
            (Var(av), bv) => self.assign(av, bv).to_iter(),
            (av, Var(bv)) => self.assign(bv, av).to_iter(),
            _ => Box::new(std::iter::empty()),
        }
    }
}

pub trait Assign<T> {
    fn assign(self, key: LVar, value: Val<T>) -> Self;
}

impl<'a, T, D: DomainType<T>> Assign<T> for State<D> {
    fn assign(mut self, key: LVar, value: Val<T>) -> Self {
        self.domain.values_as_mut().insert(key, value);
        self
    }
}
