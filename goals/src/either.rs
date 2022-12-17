use std::rc::Rc;

use canrun_core::{Fork, State};

use super::Goal;

#[derive(Clone, Debug)]
pub struct Either {
    a: Rc<dyn Goal>,
    b: Rc<dyn Goal>,
}

impl Either {
    pub fn new(a: impl Goal, b: impl Goal) -> Self {
        Either {
            a: Rc::new(a),
            b: Rc::new(b),
        }
    }
}

impl Goal for Either {
    fn apply_goal(&self, state: State) -> Option<State> {
        state.fork(self.clone())
    }
}

impl Fork for Either {
    fn fork(&self, state: &State) -> canrun_core::StateIter {
        let a = self.a.apply_goal(state.clone()).into_iter();
        let b = self.b.apply_goal(state.clone()).into_iter();
        Box::new(a.chain(b))
    }
}

#[cfg(test)]
mod test {
    use canrun_core::StateIterator;

    use crate::{fail::Fail, succeed::Succeed};

    use super::*;

    #[test]
    fn either_succeed() {
        let state = State::new();
        let goal = Either::new(Succeed::new(), Succeed::new());
        let result = Box::new(goal).apply_goal(state);
        assert_eq!(result.into_states().count(), 2);
    }

    #[test]
    fn either_succeed_or_fail() {
        let state = State::new();
        let goal = Either::new(Succeed::new(), Fail::new());
        let result = Box::new(goal).apply_goal(state);
        assert_eq!(result.into_states().count(), 1);
    }

    #[test]
    fn either_fail_or_succeed() {
        let state = State::new();
        let goal = Either::new(Fail::new(), Succeed::new());
        let result = Box::new(goal).apply_goal(state);
        assert_eq!(result.into_states().count(), 1);
    }

    #[test]
    fn either_fail() {
        let state = State::new();
        let goal = Either::new(Fail::new(), Fail::new());
        let result = Box::new(goal).apply_goal(state);
        assert_eq!(result.into_states().count(), 0);
    }
}
