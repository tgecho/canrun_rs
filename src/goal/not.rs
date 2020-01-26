use super::Goal;

pub fn not<T: Eq + Clone>(goal: Goal<T>) -> Goal<T> {
    Goal::Not(Box::new(goal))
}

#[cfg(test)]
mod tests {
    use crate::{any, both, equal, not, Cell, LVar, State};
    #[test]
    fn simple_not() {
        let state: State<u32> = State::new();
        let goal = not(equal(Cell::Value(5), Cell::Value(5)));
        let mut results = goal.run(state);
        assert_eq!(results.nth(0), None);
    }
    #[test]
    fn not_combined() {
        let x = LVar::new();
        let goal = any(vec![
            equal(x.into(), Cell::Value(1)),
            equal(x.into(), Cell::Value(2)),
            equal(x.into(), Cell::Value(3)),
        ]);
        let results: Vec<_> = goal
            .clone()
            .run(State::new())
            .map(|s| s.resolve_var(x))
            .collect();
        assert_eq!(
            results,
            vec![Cell::Value(3), Cell::Value(2), Cell::Value(1)]
        );

        let goal = both(goal, not(equal(x.into(), Cell::Value(1))));
        let results: Vec<_> = goal
            .clone()
            .run(State::new())
            .map(|s| s.resolve_var(x))
            .collect();
        assert_eq!(results, vec![Cell::Value(3), Cell::Value(2)]);
    }

    #[test]
    fn not_not() {
        let state: State<u32> = State::new();
        let x = LVar::new();
        let goal = not(not(equal(x.into(), Cell::Value(1))));
        let results: Vec<_> = goal.clone().run(state).map(|s| s.resolve_var(x)).collect();
        // I'm not actually sure if this result makes sense or is what we want
        assert_eq!(results, vec![x.into()]);

        let goal = not(not(equal(Cell::Value(1), Cell::Value(1))));
        assert!(goal.clone().run(State::new()).nth(0).is_some());
    }
}
