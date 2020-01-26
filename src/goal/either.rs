use super::Goal;

pub fn either<T: Eq + Clone>(a: Goal<T>, b: Goal<T>) -> Goal<T> {
    Goal::Either {
        a: Box::new(a),
        b: Box::new(b),
    }
}

#[cfg(test)]
mod tests {
    use crate::{either, equal, Cell, LVar, State};
    #[test]
    fn basic_either() {
        let state: State<usize> = State::new();
        let x = LVar::new();
        let xv = Cell::Var(x);
        let goal = either(equal(xv.clone(), Cell::Value(5)), equal(xv, Cell::Value(6)));
        let mut results = goal.run(&state).map(|s| s.resolve_var(x));
        assert_eq!(results.nth(0).unwrap(), Cell::Value(5));
        assert_eq!(results.nth(0).unwrap(), Cell::Value(6));
    }
}
