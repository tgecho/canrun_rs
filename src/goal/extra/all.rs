use crate::{CanT, Goal};

pub fn all<T: CanT>(goals: Vec<Goal<T>>) -> Goal<T> {
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
    use crate::{all, equal, var, Can, Equals, Goal, State};
    #[test]
    fn all_succeed() {
        let state: State<usize> = State::new();
        let x = var();
        let y = var();
        let goal = all(vec![x.equals(5), y.equals(7), Goal::Succeed]);
        let result = goal.run(state).nth(0).unwrap();
        assert_eq!(result.resolve_var(x).unwrap(), Can::Val(5));
        assert_eq!(result.resolve_var(y).unwrap(), Can::Val(7));
    }
    #[test]
    fn all_fail() {
        let state: State<usize> = State::new();
        let goal = all(vec![
            equal(Can::Val(5), Can::Val(5)),
            Goal::Succeed,
            Goal::Fail,
        ]);
        assert_eq!(goal.run(state).nth(0), None);
    }
}
