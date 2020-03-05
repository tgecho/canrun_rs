#[cfg(test)]
mod tests {
    use crate::take2::domain::{Domain, Just};
    use crate::take2::state::{State, StateIter, Unify};
    use crate::take2::val::val;

    fn run<'a, D: Domain + 'a, F: Fn(State<D>) -> Option<State<D>>>(
        func: F,
    ) -> StateIter<'a, State<'a, D>> {
        match func(State::new()) {
            None => Box::new(std::iter::empty()),
            Some(state) => state.run(),
        }
    }

    #[test]
    fn basic_equal() {
        let states = run(|s: State<Just<i32>>| s.unify(val(5), val(5)));
    }
}
