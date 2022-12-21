use std::fmt::Debug;

use crate::core::State;

pub mod both;
pub mod either;
pub mod fail;
pub mod succeed;
pub mod unify;

pub trait Goal: Debug + 'static {
    fn apply(&self, state: State) -> Option<State>;
}
