use super::Goal;
use crate::lvar::LVar;
use crate::state::State;

pub fn with<T: Eq + Clone + 'static>(
    func: Box<dyn FnOnce(LVar) -> Box<dyn Goal<T>>>,
) -> impl Goal<T> {
    WithGoal {
        func: Box::new(func),
    }
}

struct WithGoal<T: Eq + Clone> {
    func: Box<dyn FnOnce(LVar) -> Box<dyn Goal<T>>>,
}

impl<T: Eq + Clone> Goal<T> for WithGoal<T> {
    fn run<'a>(self, state: &'a State<T>) -> Box<dyn Iterator<Item = State<T>> + 'a> {
        let func = self.func;
        let goal = func(LVar::new());
        goal.run(state)
    }
}
