use super::Goal;

pub fn both<T: Eq + Clone>(a: Goal<T>, b: Goal<T>) -> Goal<T> {
    Goal::Both(BothGoal {
        a: Box::new(a),
        b: Box::new(b),
    })
}

#[derive(Clone)]
pub struct BothGoal<T: Eq + Clone + 'static> {
    pub a: Box<Goal<T>>,
    pub b: Box<Goal<T>>,
}

#[cfg(test)]
mod tests {
    use super::both;
    use crate::goal::equal;
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
