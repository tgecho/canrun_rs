use std::any::Any;
use std::fmt::Debug;
use std::rc::Rc;

use crate::State;
use crate::Value;

pub trait Unify: Any + Debug {
    fn unify(state: State, a: Rc<Self>, b: Rc<Self>) -> Option<State>;
}

impl State {
    pub fn unify<T: Unify>(mut self, a: &Value<T>, b: &Value<T>) -> Option<Self> {
        let a = self.resolve(a)?;
        let b = self.resolve(b)?;

        match (a, b) {
            (Value::Resolved(a), Value::Resolved(b)) => Unify::unify(self, a, b),
            (Value::Var(a), Value::Var(b)) if a == b => Some(self),
            (Value::Var(var), val) | (val, Value::Var(var)) => {
                // TODO: Add occurs check?
                self.values.insert(var.id, val.to_anyval());
                Some(self)
            }
        }
    }
}

impl<T: Eq + Debug + Any> Unify for T {
    fn unify(state: State, a: Rc<Self>, b: Rc<Self>) -> Option<State> {
        if a == b {
            Some(state)
        } else {
            None
        }
    }
}
