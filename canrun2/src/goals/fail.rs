use crate::core::State;
use crate::goals::Goal;

#[derive(Clone, Debug)]
pub struct Fail;

impl Goal for Fail {
    fn apply(&self, _: State) -> Option<State> {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn fail() {
        let state = State::new();
        let goal = Fail;
        let result = goal.apply(state);
        assert!(result.is_none());
    }
}
