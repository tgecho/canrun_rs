use super::Goal;
use crate::Cell;

pub fn equal<T: Eq + Clone>(a: Cell<T>, b: Cell<T>) -> Goal<T> {
    Goal::Equal { a, b }
}

#[cfg(test)]
mod tests {
    use super::equal;
    use crate::{Cell, LVar, State};
    #[test]
    fn basic_equal() {
        let state: State<u32> = State::new();
        let x = LVar::new();
        let goal = equal(Cell::Var(x), Cell::Value(5));
        let mut result = goal.run(&state);
        assert_eq!(result.nth(0).unwrap().resolve_var(x), Cell::Value(5));
    }
}
