use mkmvmap::MKMVMap;

use crate::core::Fork;
use crate::core::Unify;
use crate::value::{AnyVal, Value, VarId};
use std::rc::Rc;

use super::constrain::Constraint;

#[derive(Clone)]
pub struct State {
    pub(crate) values: im_rc::HashMap<VarId, AnyVal>,
    pub(crate) forks: im_rc::Vector<Rc<dyn Fork>>,
    constraints: MKMVMap<VarId, Rc<dyn Constraint>>,
}

impl State {
    pub fn new() -> Self {
        State {
            values: im_rc::HashMap::new(),
            forks: im_rc::Vector::new(),
            constraints: MKMVMap::new(),
        }
    }

    fn resolve_any<'a>(&'a self, val: &'a AnyVal) -> &'a AnyVal {
        match val {
            AnyVal::Var(var) => {
                let resolved = self.values.get(var);
                match resolved {
                    Some(AnyVal::Var(found_var)) if found_var == var => val,
                    Some(found) => self.resolve_any(found),
                    None => val,
                }
            }
            value => value,
        }
    }

    pub fn resolve<T: Unify>(&self, val: &Value<T>) -> Option<Value<T>> {
        self.resolve_any(&val.to_anyval()).to_value()
    }

    pub fn unify<T: Unify>(mut self, a: &Value<T>, b: &Value<T>) -> Option<Self> {
        let a = self.resolve(a)?;
        let b = self.resolve(b)?;

        match (a, b) {
            (Value::Resolved(a), Value::Resolved(b)) => Unify::unify(self, a, b),
            (Value::Var(a), Value::Var(b)) if a == b => Some(self),
            (Value::Var(key), value) | (value, Value::Var(key)) => {
                // TODO: Add occurs check?
                self.values.insert(key.id, value.to_anyval());

                // check constraints matching newly assigned lvar
                if let Some(constraints) = self.constraints.extract(&key.id) {
                    constraints
                        .into_iter()
                        .try_fold(self, |state, func| state.constrain(func))
                } else {
                    Some(self)
                }
            }
        }
    }

    pub fn fork(mut self, fork: impl Fork) -> Option<Self> {
        self.forks.push_back(Rc::new(fork));
        Some(self)
    }

    pub fn constrain(mut self, constraint: Rc<dyn Constraint>) -> Option<Self> {
        match constraint.attempt(&self) {
            Ok(resolve) => resolve(self),
            Err(watch) => {
                self.constraints.add(watch.0, constraint);
                Some(self)
            }
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use crate::{
        core::{StateIter, StateIterator},
        value::*,
    };

    use super::*;

    #[test]
    fn basic_unify() {
        let x = Value::var();
        let state = State::new();
        let state = state.unify(&x, &Value::new(1)).unwrap();
        assert_eq!(state.resolve(&x).unwrap(), Value::new(1));
    }

    #[test]
    fn basic_fork() {
        let x = LVar::new();
        let state: State = State::new();
        let results = state
            .fork(move |s: &State| -> StateIter {
                let s1 = s.clone().unify(&x.into(), &Value::new(1));
                let s2 = s.clone().unify(&x.into(), &Value::new(2));
                Box::new(s1.into_iter().chain(s2.into_iter()))
            })
            .into_states()
            .map(|s| s.resolve(&x.into()).unwrap())
            .collect::<Vec<_>>();
        assert_eq!(results, vec![Value::new(1), Value::new(2)]);
    }
}
