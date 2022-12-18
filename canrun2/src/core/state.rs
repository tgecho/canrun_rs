use crate::core::value::{AnyVal, Value, VarId};
use crate::Fork;
use crate::Unify;
use std::rc::Rc;

#[derive(Clone)]
pub struct State {
    pub(crate) values: im_rc::HashMap<VarId, AnyVal>,
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
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use crate::{
        core::value::*,
        {StateIter, StateIterator},
    };

    use super::*;

    #[test]
    fn basic_unify() {
        let x = Value::var();
        let state = State::new();
        let state = state.unify(x.clone(), Value::new(1)).unwrap();
        assert_eq!(state.resolve(&x).unwrap(), Value::new(1));
    }

    #[test]
    fn basic_fork() {
        let x = LVar::new();
        let state: State = State::new();
        let results = state
            .fork(move |s: &State| -> StateIter {
                let s1 = s.clone().unify(x.into(), Value::new(1));
                let s2 = s.clone().unify(x.into(), Value::new(2));
                Box::new(s1.into_iter().chain(s2.into_iter()))
            })
            .into_states()
            .map(|s| s.resolve(&x.into()).unwrap())
            .collect::<Vec<_>>();
        assert_eq!(results, vec![Value::new(1), Value::new(2)]);
    }
}
