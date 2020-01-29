use crate::{CanT, Goal};

pub fn any<T: CanT>(goals: Vec<Goal<T>>) -> Goal<T> {
    let mut iter = goals.into_iter();
    match iter.next() {
        Some(first) => iter.fold(first, |a, b| Goal::Either {
            a: Box::new(b),
            b: Box::new(a),
        }),
        None => Goal::Succeed,
    }
}

#[cfg(test)]
mod tests {
    use crate::{any, equal, Can, Goal, LVar, State};
    #[test]
    fn any_succeed() {
        let state: State<usize> = State::new();
        let x = LVar::new();
        let y = LVar::new();
        let goal = any(vec![
            Goal::Fail,
            equal(x.into(), Can::Val(5)),
            equal(y.into(), Can::Val(7)),
        ]);
        let results: Vec<State<usize>> = goal.run(state).collect();
        assert_eq!(results[0].resolve_var(y), Can::Val(7));
        assert_eq!(results[1].resolve_var(x), Can::Val(5));
    }
    #[test]
    fn any_fail() {
        let state: State<usize> = State::new();
        let goal = any(vec![equal(Can::Val(5), Can::Val(6)), Goal::Fail]);
        assert_eq!(goal.run(state).nth(0), None);
    }
}
