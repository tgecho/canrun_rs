#[cfg(test)]
mod tests {
    use crate::take2::domain::{Domain, Just};
    use crate::take2::state::{State, StateIter};
    use crate::take2::val::val;

    fn run<'a, D: Domain + 'a, F: Fn(State<D>) -> Result<State<D>, State<D>>>(
        func: F,
    ) -> StateIter<'a, State<'a, D>> {
        match func(State::new()) {
            Err(_) => Box::new(std::iter::empty()),
            Ok(state) => state.run(),
        }
    }

    #[test]
    fn basic_equal() {
        let states = run(|s: State<Just<i32>>| s.unify(val(5), val(5)));
    }
}
