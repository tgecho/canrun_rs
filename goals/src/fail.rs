use canrun_core::State;

use crate::Goal;

#[derive(Debug, PartialEq, Eq)]
pub struct Fail;

impl Fail {
    pub fn new() -> Fail {
        Fail
    }
}

impl Goal for Fail {
    fn apply_goal(&self, _: State) -> Option<State> {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn fail() {
        let state = State::new();
        let goal = Fail::new();
        let result = goal.apply_goal(state);
        assert!(result.is_none());
    }
}
