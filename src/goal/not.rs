use crate::state::empty_iter;
use crate::{CanT, Goal, State, StateIter};

pub fn not<T: CanT>(goal: Goal<T>) -> Goal<T> {
    Goal::Not(Box::new(goal))
}

pub(crate) fn run<'a, T: CanT + 'a>(state: State<T>, goal: Goal<T>) -> StateIter<'a, T> {
    if goal.run(state.clone()).nth(0).is_some() {
        empty_iter()
    } else {
        state.to_iter()
    }
}

#[cfg(test)]
mod tests {
    use crate::{any, both, equal, not, var, Can, Equals, Goal, State};
    #[test]
    fn simple_not() {
        let state: State<u32> = State::new();
        let goal = not(equal(Can::Val(5), Can::Val(5)));
        let mut results = goal.run(state);
        assert_eq!(results.nth(0), None);
    }
    #[test]
    fn not_combined() {
        let x = var();
        let goal = any(vec![x.equals(1), x.equals(2), x.equals(3)]);
        let results: Vec<_> = goal
            .clone()
            .run(State::new())
            .map(|s| s.resolve_var(x).unwrap())
            .collect();
        assert_eq!(results, vec![Can::Val(3), Can::Val(2), Can::Val(1)]);

        let goal = both(goal, not(x.equals(1)));
        let results: Vec<_> = goal
            .clone()
            .run(State::new())
            .map(|s| s.resolve_var(x).unwrap())
            .collect();
        assert_eq!(results, vec![Can::Val(3), Can::Val(2)]);
    }

    #[test]
    fn not_not() {
        let state: State<u32> = State::new();
        let x = var();
        let goal = not(not(x.equals(1)));
        let results: Vec<_> = goal.run(state).map(|s| s.resolve_var(x).unwrap()).collect();
        // I'm not actually sure if this result makes sense or is what we want
        assert_eq!(results, vec![x.can()]);

        // Not sure why this can't infer T :/
        let goal: Goal<usize> = not(not(equal(Can::Val(1), Can::Val(1))));
        assert!(goal.run(State::new()).nth(0).is_some());
    }
}
