use std::any::Any;
use std::fmt::Debug;
use std::rc::Rc;

use crate::core::State;

pub trait Unify: Any + Debug {
    fn unify(state: State, a: Rc<Self>, b: Rc<Self>) -> Option<State>;
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
