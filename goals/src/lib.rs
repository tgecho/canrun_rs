use std::fmt::Debug;

use canrun_core::State;

pub mod both;
pub mod fail;
pub mod succeed;

pub trait Goal: Debug + 'static {
    fn apply_goal(&self, state: State) -> Option<State>;
}
