use crate::goals::Goal;
use crate::State;

#[derive(Debug)]
pub struct Succeed;

impl Succeed {
    pub fn new() -> Succeed {
        Succeed
    }
}

impl Default for Succeed {
    fn default() -> Self {
        Self::new()
    }
}

impl Goal for Succeed {
    fn apply_goal(&self, state: State) -> Option<State> {
        Some(state)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn succeed() {
        let state = State::new();
        let goal = Succeed::new();
        let result = goal.apply_goal(state);
        assert!(result.is_some());
    }
}
