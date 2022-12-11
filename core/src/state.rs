use crate::state_iterator::StateIter;
use crate::unify::Unify;
use crate::value::{AnyVal, LVarId, Val};

use std::rc::Rc;

pub trait Fork: 'static {
    /// Given a [`State`], return an iterator of states that result from the
    /// fork operation.
    fn fork(&self, state: State) -> StateIter;
}

impl Fork for Rc<dyn Fn(State) -> StateIter> {
    fn fork(&self, state: State) -> StateIter {
        self(state)
    }
}

#[derive(Clone)]
pub struct State {
    values: im_rc::HashMap<LVarId, AnyVal>,
    pub(crate) forks: im_rc::Vector<Rc<dyn Fork>>,
}

impl State {
    pub fn new() -> Self {
        State {
            values: im_rc::HashMap::new(),
            forks: im_rc::Vector::new(),
        }
    }

    fn resolve_any<'a>(&'a self, val: &'a AnyVal) -> &'a AnyVal {
        match val {
            AnyVal::Var(var) => {
                let resolved = self.values.get(&var.id);
                match resolved {
                    Some(AnyVal::Var(found_var)) if found_var == var => val,
                    Some(found) => self.resolve_any(found),
                    None => val,
                }
            }
            value => value,
        }
    }

    pub fn resolve<T: Unify>(&self, val: &Val<T>) -> Option<Val<T>> {
        match self.resolve_any(&AnyVal::from(val)) {
            AnyVal::Var(var) => Some(Val::Var(*var)),
            AnyVal::Value(val) => {
                let rc_t = val.clone().downcast::<T>().ok()?;
                Some(Val::Value(rc_t))
            }
        }
    }

    pub fn unify<T: Unify>(mut self, a: Val<T>, b: Val<T>) -> Option<Self> {
        let a = self.resolve(&a)?;
        let b = self.resolve(&b)?;

        match (a, b) {
            (Val::Value(a), Val::Value(b)) => Unify::unify(self, a, b),
            (Val::Var(a), Val::Var(b)) if a == b => Some(self),
            (Val::Var(var), val) | (val, Val::Var(var)) => {
                // TODO: Add occurs check?
                self.values.insert(var.id, AnyVal::from(val));
                Some(self)
            }
        }
    }

    pub fn fork<F: Fork>(mut self, fork: Rc<F>) -> Option<Self> {
        self.forks.push_back(fork);
        Some(self)
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use crate::{state_iterator::StateIterator, value::*};

    use super::*;

    #[test]
    fn basic_unify() {
        let x = var();
        let state = State::new();

        let state = state.unify(x.clone(), val(1)).unwrap();
        assert_eq!(state.resolve(&x).unwrap(), val(1));
    }

    struct Is1or2 {
        x: Val<usize>,
    }

    impl Fork for Is1or2 {
        fn fork(&self, state: State) -> StateIter {
            let s1 = state.clone().unify(self.x.clone(), val(1));
            let s2 = state.unify(self.x.clone(), val(2));
            Box::new(s1.into_iter().chain(s2.into_iter()))
        }
    }

    #[test]
    fn basic_fork() {
        let x = var();
        let state: State = State::new();

        let results = state
            .fork(Rc::new(Is1or2 { x: x.clone() }))
            .into_states()
            .map(|s| s.resolve(&x).unwrap())
            .collect::<Vec<_>>();
        assert_eq!(results, vec![val(1), val(2)]);
    }
}
