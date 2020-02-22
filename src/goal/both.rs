use crate::{CanT, Goal, State, StateIter};
use std::iter::once;

pub fn both<'a, T: CanT>(a: Goal<'a, T>, b: Goal<'a, T>) -> Goal<'a, T> {
    Goal::Both {
        a: Box::new(a),
        b: Box::new(b),
    }
}

pub(crate) fn run<'a, T: CanT + 'a>(
    state: State<'a, T>,
    a: Goal<'a, T>,
    b: Goal<'a, T>,
) -> StateIter<'a, T> {
    Box::new(
        a.run(state)
            .zip(once(b).cycle())
            .flat_map(|(s, b)| b.run(s)),
    )
}

#[cfg(test)]
mod tests {
    use crate::{both, var, Can, Equals, State};
    #[test]
    fn basic_both() {
        let state: State<usize> = State::new();
        let x = var();
        let y = var();
        let goal = both(x.equals(Can::Val(5)), y.equals(Can::Val(7)));
        let result = goal.run(state).nth(0).unwrap();
        assert_eq!(result.resolve_var(x).unwrap(), Can::Val(5));
        assert_eq!(result.resolve_var(y).unwrap(), Can::Val(7));
    }
}
