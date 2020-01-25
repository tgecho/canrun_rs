use super::Goal;

pub fn all<T: Eq + Clone>(goals: Vec<Goal<T>>) -> Goal<T> {
    let mut iter = goals.into_iter();
    match iter.next() {
        Some(first) => iter.fold(first, |a, b| Goal::Both {
            a: Box::new(a),
            b: Box::new(b),
        }),
        None => Goal::Succeed,
    }
}

#[cfg(test)]
mod tests {
    use super::all;
    use crate::goal::{equal, Goal};
    use crate::{Cell, LVar, State};
    #[test]
    fn all_succeed() {
        let state: State<usize> = State::new();
        let x = LVar::new();
        let y = LVar::new();
        let goal = all(vec![
            equal(x.into(), Cell::Value(5)),
            equal(y.into(), Cell::Value(7)),
            Goal::Succeed,
        ]);
        let result = goal.run(&state).nth(0).unwrap();
        assert_eq!(result.resolve_var(x), Cell::Value(5));
        assert_eq!(result.resolve_var(y), Cell::Value(7));
    }
    #[test]
    fn all_fail() {
        let state: State<usize> = State::new();
        let goal = all(vec![
            equal(Cell::Value(5), Cell::Value(5)),
            Goal::Succeed,
            Goal::Fail,
        ]);
        assert_eq!(goal.run(&state).nth(0), None);
    }
}
