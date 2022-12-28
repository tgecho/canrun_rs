use crate::core::State;
use crate::goals::Goal;

#[derive(Debug)]
pub struct Succeed;

impl Goal for Succeed {
    fn apply(&self, state: State) -> Option<State> {
        Some(state)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn succeed() {
        let state = State::new();
        let goal = Succeed;
        let result = goal.apply(state);
        assert!(result.is_some());
    }
}
