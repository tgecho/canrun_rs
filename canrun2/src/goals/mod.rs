use std::{fmt::Debug, rc::Rc};

use crate::core::State;

pub mod all;
pub mod any;
pub mod both;
pub mod either;
pub mod fail;
pub mod lazy;
pub mod project;
pub mod succeed;
pub mod unify;

pub trait Goal: Debug + 'static {
    fn apply(&self, state: State) -> Option<State>;
}

impl Goal for Rc<dyn Goal> {
    fn apply(&self, state: State) -> Option<State> {
        self.as_ref().apply(state)
    }
}
