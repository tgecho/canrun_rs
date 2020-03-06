#[cfg(test)]
mod tests {
    use crate::take2::core::domain::Just;
    use crate::take2::core::state::{run, State};
    use crate::take2::core::val::val;

    #[test]
    fn basic_equal() {
        let states = run(|s: State<Just<i32>>| s.unify(val(5), val(5)));
    }
}
