use std::any::Any;
use std::fmt::Debug;
use std::rc::Rc;

use crate::state::State;

pub trait Unify: Any + Debug {
    fn unify(state: State, a: Rc<Self>, b: Rc<Self>) -> Option<State>;
}

impl Unify for usize {
    fn unify(state: State, a: Rc<Self>, b: Rc<Self>) -> Option<State> {
        if a == b {
            Some(state)
        } else {
            None
        }
    }
}
