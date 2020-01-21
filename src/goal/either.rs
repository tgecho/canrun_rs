use super::Goal;
use crate::lvar::LVar;
use crate::state::State;

pub fn either<T: Eq + Clone, G: Goal<T>>(a: G, b: G) -> impl Goal<T> {
    EitherGoal { a, b }
}

struct EitherGoal<G> {
    a: G,
    b: G,
}

impl<T: Eq + Clone, G: Goal<T>> Goal<T> for EitherGoal<G> {
    fn run<'a>(self, state: &'a State<T>) -> Box<dyn Iterator<Item = State<T>> + 'a> {
        Box::new(self.a.run(&state).chain(self.b.run(&state)))
    }
}

#[cfg(test)]
mod tests {
    use super::{either, Goal};
    use crate::goal::equal::equal;
    use crate::lvar::LVar;
    use crate::state::{Cell, State};
    #[test]
    fn basic_equal() {
        let state: State<usize> = State::new();
        let x = LVar::new();
        let xv = Cell::Var(x);
        let goal = either(equal(xv.clone(), Cell::Value(5)), equal(xv, Cell::Value(6)));
        let mut results = goal.run(&state).map(|s| s.resolve_var(x));
        // .collect::<Vec<Cell<usize>>>();
        assert_eq!(results.nth(0).unwrap(), Cell::Value(5));
        assert_eq!(results.nth(0).unwrap(), Cell::Value(6));
    }
}
