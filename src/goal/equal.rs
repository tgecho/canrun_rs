use crate::{Can, CanT, Goal, State, StateIter};

pub fn equal<T: CanT>(a: Can<T>, b: Can<T>) -> Goal<T> {
    Goal::Equal { a, b }
}

pub(crate) fn run<T: CanT + 'static>(state: &State<T>, a: &Can<T>, b: &Can<T>) -> StateIter<T> {
    Box::new(state.unify(a, b))
}

#[cfg(test)]
mod tests {
    use super::equal;
    use crate::{Can, LVar, State};
    #[test]
    fn basic_equal() {
        let state: State<u32> = State::new();
        let x = LVar::new();
        let goal = equal(Can::Var(x), Can::Val(5));
        let mut result = goal.run(&state);
        assert_eq!(result.nth(0).unwrap().resolve_var(x).unwrap(), Can::Val(5));
    }
}
