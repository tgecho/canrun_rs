use std::rc::Rc;

use crate::core::{Fork, State, StateIter};

use super::Goal;

#[derive(Clone, Debug)]
pub struct Either {
    a: Rc<dyn Goal>,
    b: Rc<dyn Goal>,
}

pub fn either(a: impl Goal, b: impl Goal) -> Either {
    Either {
        a: Rc::new(a),
        b: Rc::new(b),
    }
}

impl Goal for Either {
    fn apply(&self, state: State) -> Option<State> {
        state.fork(self.clone())
    }
}

impl Fork for Either {
    fn fork(&self, state: &State) -> StateIter {
        let a = self.a.apply(state.clone()).into_iter();
        let b = self.b.apply(state.clone()).into_iter();
        Box::new(a.chain(b))
    }
}

#[cfg(test)]
mod test {
    use crate::core::StateIterator;

    use crate::goals::{fail::Fail, succeed::Succeed};

    use super::*;

    #[test]
    fn either_succeed() {
        let state = State::new();
        let goal = either(Succeed, Succeed);
        let result = Box::new(goal).apply(state);
        assert_eq!(result.into_states().count(), 2);
    }

    #[test]
    fn either_succeed_or_fail() {
        let state = State::new();
        let goal = either(Succeed, Fail);
        let result = Box::new(goal).apply(state);
        assert_eq!(result.into_states().count(), 1);
    }

    #[test]
    fn either_fail_or_succeed() {
        let state = State::new();
        let goal = either(Fail, Succeed);
        let result = Box::new(goal).apply(state);
        assert_eq!(result.into_states().count(), 1);
    }

    #[test]
    fn either_fail() {
        let state = State::new();
        let goal = either(Fail, Fail);
        let result = Box::new(goal).apply(state);
        assert_eq!(result.into_states().count(), 0);
    }
}
