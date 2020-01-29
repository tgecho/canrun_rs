use crate::{CanT, Goal};

pub fn either<T: CanT>(a: Goal<T>, b: Goal<T>) -> Goal<T> {
    Goal::Either {
        a: Box::new(a),
        b: Box::new(b),
    }
}

#[cfg(test)]
mod tests {
    use crate::{either, equal, Can, LVar, State};
    #[test]
    fn basic_either() {
        let state: State<usize> = State::new();
        let x = LVar::new();
        let xv = Can::Var(x);
        let goal = either(equal(xv.clone(), Can::Val(5)), equal(xv, Can::Val(6)));
        let mut results = goal.run(state).map(|s| s.resolve_var(x));
        assert_eq!(results.nth(0).unwrap(), Can::Val(5));
        assert_eq!(results.nth(0).unwrap(), Can::Val(6));
    }
}
