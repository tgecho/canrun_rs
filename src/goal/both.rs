use crate::{CanT, Goal};

pub fn both<T: CanT>(a: Goal<T>, b: Goal<T>) -> Goal<T> {
    Goal::Both {
        a: Box::new(a),
        b: Box::new(b),
    }
}

#[cfg(test)]
mod tests {
    use crate::{both, equal, Can, LVar, State};
    #[test]
    fn basic_both() {
        let state: State<usize> = State::new();
        let x = LVar::new();
        let xv = Can::Var(x);
        let y = LVar::new();
        let yv = Can::Var(y);
        let goal = both(equal(xv.clone(), Can::Val(5)), equal(yv, Can::Val(7)));
        let result = goal.run(state).nth(0).unwrap();
        assert_eq!(result.resolve_var(x), Can::Val(5));
        assert_eq!(result.resolve_var(y), Can::Val(7));
    }
}
