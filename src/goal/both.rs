use super::Goal;
use crate::state::State;

pub fn both<'a, T: Eq + Clone, G: Goal<T> + 'a>(a: G, b: G) -> impl Goal<T> {
    BothGoal { a, b }
}

#[derive(Clone)]
struct BothGoal<G> {
    a: G,
    b: G,
}

impl<T: Eq + Clone, G: Goal<T>> Goal<T> for BothGoal<G> {
    fn run<'a>(self, state: &'a State<T>) -> Box<dyn Iterator<Item = State<T>> + 'a> {
        let a_states = self.a.run(&state).collect::<Vec<State<T>>>();
        let ab_states = a_states.iter().flat_map(|a| self.b.run(a));
        Box::new(ab_states)
    }
}

#[cfg(test)]
mod tests {
    use super::{both, Goal};
    use crate::goal::equal::equal;
    use crate::lvar::LVar;
    use crate::state::{Cell, State};
    #[test]
    fn basic_both() {
        let state: State<usize> = State::new();
        let x = LVar::new();
        let xv = Cell::Var(x);
        let y = LVar::new();
        let yv = Cell::Var(y);
        let goal = both(equal(xv.clone(), Cell::Value(5)), equal(yv, Cell::Value(7)));
        let result = goal.run(&state).nth(0).unwrap();
        assert_eq!(result.resolve_var(x), Cell::Value(5));
        assert_eq!(result.resolve_var(y), Cell::Value(7));
    }
}
