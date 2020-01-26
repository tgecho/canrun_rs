use super::Goal;
use crate::Cell;

pub fn member<T: Eq + Clone>(needle: Cell<T>, haystack: Vec<Cell<T>>) -> Goal<T> {
    Goal::Member { needle, haystack }
}

#[cfg(test)]
mod tests {
    use super::member;
    use crate::{both, equal, Cell, LVar, State};
    #[test]
    fn basic_member() {
        let x = LVar::new();
        let goal = member(
            Cell::Var(x),
            vec![Cell::Value(1), Cell::Value(2), Cell::Value(3)],
        );
        let result: Vec<_> = goal.run(State::new()).map(|r| r.resolve_var(x)).collect();
        assert_eq!(result, vec![Cell::Value(1), Cell::Value(2), Cell::Value(3)]);
    }
    #[test]
    fn member_with_conditions() {
        let x = LVar::new();
        let goal = both(
            equal(Cell::Var(x), Cell::Value(2)),
            member(
                Cell::Var(x),
                vec![Cell::Value(1), Cell::Value(2), Cell::Value(3)],
            ),
        );
        let result: Vec<_> = goal.run(State::new()).map(|r| r.resolve_var(x)).collect();
        assert_eq!(result, vec![Cell::Value(2)]);
    }
}
