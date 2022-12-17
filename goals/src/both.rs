use canrun_core::State;

use super::Goal;

#[derive(Debug)]
pub struct Both {
    a: Box<dyn Goal>,
    b: Box<dyn Goal>,
}

impl Both {
    pub fn new(a: impl Goal, b: impl Goal) -> Both {
        Both {
            a: Box::new(a),
            b: Box::new(b),
        }
    }
}

impl Goal for Both {
    fn apply_goal(&self, state: State) -> Option<State> {
        self.a.apply_goal(state).and_then(|s| self.b.apply_goal(s))
    }
}

#[cfg(test)]
mod test {
    use crate::{fail::Fail, succeed::Succeed};

    use super::*;

    #[test]
    fn both_succeed() {
        let state = State::new();
        let goal = Both::new(Succeed::new(), Succeed::new());
        let result = goal.apply_goal(state);
        assert!(result.is_some());
    }

    #[test]
    fn both_succeed_then_fail() {
        let state = State::new();
        let goal = Both::new(Succeed::new(), Fail::new());
        let result = goal.apply_goal(state);
        assert!(result.is_none());
    }

    #[test]
    fn both_fail_then_succeed() {
        let state = State::new();
        let goal = Both::new(Fail::new(), Succeed::new());
        let result = goal.apply_goal(state);
        assert!(result.is_none());
    }
}
