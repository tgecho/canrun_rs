use super::domain::{Domain, DomainType};
use super::val::{Val, Val::Var};
use crate::can::lvar::LVar;
use crate::util::multikeymultivaluemap::MKMVMap;
use std::rc::Rc;

pub type StateIter<'s, State> = Box<dyn Iterator<Item = State> + 's>;

#[derive(Clone)]
pub struct State<'a, D: Domain> {
    domain: D,
    watches: MKMVMap<LVar, Rc<dyn Fn(Self) -> WatchResult<Self> + 'a>>,
    forks: im::Vector<Rc<dyn Fn(Self) -> StateIter<'a, Self> + 'a>>,
}

pub(crate) enum WatchResult<State> {
    Done(Result<State, State>),
    Waiting(State, Vec<LVar>), // TODO: does this need to be by T row?
}

pub fn run<'a, D: Domain + 'a, F: Fn(State<D>) -> Result<State<D>, State<D>>>(
    func: F,
) -> StateIter<'a, State<'a, D>> {
    match func(State::new()) {
        Err(_) => Box::new(std::iter::empty()),
        Ok(state) => state.iter(),
    }
}

impl<'a, D: Domain + 'a> State<'a, D> {
    pub fn new() -> Self {
        State {
            domain: D::new(),
            watches: MKMVMap::new(),
            forks: im::Vector::new(),
        }
    }

    pub fn apply<F>(self, func: F) -> Result<Self, Self>
    where
        F: Fn(Self) -> Result<Self, Self>,
    {
        func(self)
    }

    pub(crate) fn iter(&self) -> StateIter<'a, Self> {
        let mut state = self.clone();
        let fork = state.forks.pop_front();
        match fork {
            None => Box::new(std::iter::once(state)),
            Some(fork) => Box::new(fork(state).flat_map(|s| s.iter())),
        }
    }

    pub(crate) fn resolve<'r, T>(&'r self, key: &'r Val<T>) -> &'r Val<T>
    where
        D: DomainType<T>,
    {
        match key {
            Val::Var(var) => self.domain.values_as_ref().get(var).unwrap_or(key),
            value => value,
        }
    }

    pub(crate) fn unify<T>(mut self, a: Val<T>, b: Val<T>) -> Result<Self, Self>
    where
        T: PartialEq,
        D: DomainType<T>,
    {
        let a = self.resolve(&a);
        let b = self.resolve(&b);
        match (a, b) {
            (a, b) if a == b => Ok(self),
            (Var(var), val) | (val, Var(var)) => {
                let key = *var;
                let value = val.clone();

                // TODO: Add occurs check?

                // Assign lvar to value
                self.domain.values_as_mut().insert(key, value);

                // check watches matching newly assigned lvar
                let (watches, extracted) = self.watches.extract(&key);
                self.watches = watches;
                (extracted.into_iter()).try_fold(self, |state, func| state.watch(func))
            }
            _ => Err(self),
        }
    }

    pub(crate) fn watch<T>(
        self,
        func: Rc<dyn Fn(Self) -> WatchResult<Self> + 'a>,
    ) -> Result<Self, Self>
    where
        D: DomainType<T>,
    {
        match func(self) {
            WatchResult::Done(state) => state,
            WatchResult::Waiting(state, vars) => {
                state.watches.add(vars, func);
                Ok(state)
            }
        }
    }

    pub(crate) fn fork<T, F>(mut self, func: F) -> Result<Self, Self>
    where
        D: DomainType<T>,
        F: Fn(Self) -> StateIter<'a, Self> + 'a,
    {
        self.forks.push_back(Rc::new(func));
        Ok(self)
    }
}