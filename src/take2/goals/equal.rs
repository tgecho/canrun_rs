#[cfg(test)]
mod tests {
    use crate::take2::domain::Just;
    use crate::take2::state::{State, Unify};
    use crate::take2::val::val;

    #[test]
    fn basic_equal() {
        let state: State<Just<i32>> = State::new();
        let state = state.unify(val(5), val(5));
    }
}
