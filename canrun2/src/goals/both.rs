use super::Goal;
use crate::core::State;

#[derive(Debug)]
pub struct Both {
    a: Box<dyn Goal>,
    b: Box<dyn Goal>,
}

pub fn both(a: impl Goal, b: impl Goal) -> Both {
    Both {
        a: Box::new(a),
        b: Box::new(b),
    }
}

impl Goal for Both {
    fn apply(&self, state: State) -> Option<State> {
        self.a.apply(state).and_then(|s| self.b.apply(s))
    }
}

#[cfg(test)]
mod test {
    use crate::goals::{fail::Fail, succeed::Succeed};

    use super::*;

    #[test]
    fn both_succeed() {
        let state = State::new();
        let goal = both(Succeed, Succeed);
        let result = goal.apply(state);
        assert!(result.is_some());
    }

    #[test]
    fn both_succeed_then_fail() {
        let state = State::new();
        let goal = both(Succeed, Fail);
        let result = goal.apply(state);
        assert!(result.is_none());
    }

    #[test]
    fn both_fail_then_succeed() {
        let state = State::new();
        let goal = both(Fail, Succeed);
        let result = goal.apply(state);
        assert!(result.is_none());
    }
}
