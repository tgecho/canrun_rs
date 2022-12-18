use std::fmt::Debug;

use crate::State;

pub mod both;
pub mod either;
pub mod fail;
pub mod succeed;

pub trait Goal: Debug + 'static {
    fn apply_goal(&self, state: State) -> Option<State>;
}
