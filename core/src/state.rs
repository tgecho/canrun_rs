mod impls;
mod iter_resolved;
mod resolved;

use super::util::multikeymultivaluemap::MKMVMap;
use crate::domains::{Domain, DomainType};
use crate::unify::Unify;
use crate::value::{
    LVar, LVarId, Val,
    Val::{Resolved, Var},
};
pub use iter_resolved::IterResolved;
pub use resolved::ResolvedState;
use std::iter::once;
use std::rc::Rc;

pub type StateIter<'s, D> = Box<dyn Iterator<Item = State<'s, D>> + 's>;
type WatchFns<'s, D> = MKMVMap<LVarId, Rc<dyn Fn(State<'s, D>) -> Watch<State<'s, D>> + 's>>;
#[doc(hidden)]
pub use im::HashMap;

#[derive(Clone)]
pub struct State<'a, D: Domain<'a> + 'a> {
    domain: D,
    watches: WatchFns<'a, D>,
    forks: im::Vector<Rc<dyn Fn(Self) -> StateIter<'a, D> + 'a>>,
}

#[derive(Debug)]
pub struct WatchList(Vec<LVarId>);

#[derive(Debug)]
pub enum Watch<State> {
    Done(Option<State>),
    Waiting(State, WatchList),
}

impl<S> Watch<S> {
    pub fn done(state: Option<S>) -> Self {
        Watch::Done(state)
    }
    pub fn watch<T>(state: S, var: LVar<T>) -> Watch<S> {
        Watch::Waiting(state, WatchList(vec![var.id]))
    }
    pub fn and<T>(self, var: LVar<T>) -> Watch<S> {
        match self {
            Watch::Done(Some(state)) => Watch::watch(state, var),
            Watch::Done(None) => self,
            Watch::Waiting(state, mut list) => {
                list.0.push(var.id);
                Watch::Waiting(state, list)
            }
        }
    }
}

impl<'a, D: Domain<'a> + 'a> State<'a, D> {
    pub fn new() -> Self {
        State {
            domain: D::new(),
            watches: MKMVMap::new(),
            forks: im::Vector::new(),
        }
    }

    pub fn apply<F>(self, func: F) -> Option<Self>
    where
        F: Fn(Self) -> Option<Self>,
    {
        func(self)
    }

    fn iter_forks(mut self) -> StateIter<'a, D> {
        let fork = self.forks.pop_front();
        match fork {
            None => Box::new(once(self)),
            Some(fork) => Box::new(fork(self).flat_map(|s: State<'a, D>| s.iter_forks())),
        }
    }

    pub fn resolve_val<'r, T>(&'r self, val: &'r Val<T>) -> &'r Val<T>
    where
        D: DomainType<'a, T>,
    {
        match val {
            Val::Var(var) => self.domain.values_as_ref().get(var).unwrap_or(val),
            value => value,
        }
    }

    pub fn get<'g, T>(&'g self, var: LVar<T>) -> Result<&'g T, LVar<T>>
    where
        D: DomainType<'a, T>,
    {
        match self.domain.values_as_ref().get(&var) {
            Some(val) => val.resolved(),
            None => Err(var),
        }
    }

    pub fn unify<T>(mut self, a: Val<T>, b: Val<T>) -> Option<Self>
    where
        D: DomainType<'a, T>,
        Self: Unify<'a, T>,
    {
        let a = self.resolve_val(&a);
        let b = self.resolve_val(&b);
        match (a, b) {
            (Resolved(a), Resolved(b)) => {
                let a = a.clone();
                let b = b.clone();
                self.unify_resolved(a, b)
            }
            (Var(a), Var(b)) if a == b => Some(self),
            (Var(var), val) | (val, Var(var)) => {
                let key = *var;
                let value = val.clone();

                // TODO: Add occurs check?

                // Assign lvar to value
                self.domain.values_as_mut().insert(key, value);

                // check watches matching newly assigned lvar
                if let Some(watches) = self.watches.extract(&key.id) {
                    watches
                        .into_iter()
                        .try_fold(self, |state, func| state.watch(func))
                } else {
                    Some(self)
                }
            }
        }
    }

    pub fn watch(self, func: Rc<dyn Fn(Self) -> Watch<Self> + 'a>) -> Option<Self> {
        match func(self) {
            Watch::Done(state) => state,
            Watch::Waiting(mut state, WatchList(vars)) => {
                state.watches.add(vars, func);
                Some(state)
            }
        }
    }

    pub fn fork(mut self, func: Rc<dyn Fn(Self) -> StateIter<'a, D> + 'a>) -> Option<Self> {
        self.forks.push_back(func);
        Some(self)
    }
}
