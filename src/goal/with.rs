use super::Goal;
use crate::lvar::LVar;
use crate::state::State;

pub fn with<T: Eq + Clone + 'static, G: Goal<T>>(func: fn(LVar) -> G) -> impl Goal<T> {
    WithGoal { func }
}

#[derive(Clone)]
struct WithGoal<G> {
    func: fn(LVar) -> G,
}

impl<T: Eq + Clone, G: Goal<T>> Goal<T> for WithGoal<G> {
    fn run<'a>(self, state: &'a State<T>) -> Box<dyn Iterator<Item = State<T>> + 'a> {
        let goal = self.func;
        goal(LVar::new()).run(state)
    }
}

#[cfg(test)]
mod tests {
    use super::with;
    use crate::goal::equal::equal;
    use crate::goal::Goal;
    use crate::state::{Cell, State};
    #[test]
    fn basic_with() {
        let state: State<u32> = State::new();
        let goal = with(|x| equal(Cell::Var(x), Cell::Value(5)));
        let mut result = goal.run(&state);
        // TODO: Use one of the fancier (as of yet unimplemented) goals to
        // inject a variable we can use to check this result
        assert!(result.nth(0).is_some());
    }
}
