use crate::{CanT, Goal, State, StateIter};
use itertools::Itertools;

pub fn either<T: CanT>(a: Goal<T>, b: Goal<T>) -> Goal<T> {
    Goal::Either {
        a: Box::new(a),
        b: Box::new(b),
    }
}

pub(crate) fn run<T: CanT>(state: &State<T>, a: &Goal<T>, b: &Goal<T>) -> StateIter<T> {
    Box::new(a.run(state).interleave(b.run(state)))
}

#[cfg(test)]
mod tests {
    use crate::{either, Can, LVar, State, Equals};
    #[test]
    fn basic_either() {
        let state: State<usize> = State::new();
        let x = LVar::new();
        let goal = either(x.equals(Can::Val(5)), x.equals(Can::Val(6)));
        let mut results = goal.run(&state).map(|s| s.resolve_var(x).unwrap());
        assert_eq!(results.nth(0).unwrap(), Can::Val(5));
        assert_eq!(results.nth(0).unwrap(), Can::Val(6));
    }
}
