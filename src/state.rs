use crate::can::lvar::LVar;
use crate::can::{Can, CanT};
use crate::unify::Unify;
use im::hashmap::HashMap;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct State<T: CanT> {
    values: HashMap<LVar, Can<T>>,
}

impl<T: CanT> State<T> {
    pub fn new() -> State<T> {
        State {
            values: HashMap::new(),
        }
    }

    pub fn assign(&self, key: LVar, value: Can<T>) -> Self {
        State {
            values: self.values.update(key, value),
        }
    }

    pub fn resolve(&self, can: &Can<T>) -> Can<T> {
        match can {
            Can::Var(lvar) => match self.values.get(lvar) {
                Some(val) => self.resolve(val),
                None => can.clone(),
            },
            Can::Val(_) => can.clone(),
            Can::Pair(p) => p.resolve_in(self),
            Can::Vec(v) => v.resolve_in(self),
            Can::Nil => Can::Nil,
        }
    }

    pub fn resolve_var(&self, key: LVar) -> Can<T> {
        self.resolve(&Can::Var(key))
    }

    pub fn unify(&self, a: &Can<T>, b: &Can<T>) -> Option<State<T>> {
        let a = self.resolve(a);
        let b = self.resolve(b);

        if a == b {
            Some(self.clone())
        } else {
            match (a, b) {
                (Can::Var(av), bv) => Some(self.assign(av, bv)),
                (av, Can::Var(bv)) => Some(self.assign(bv, av)),
                (Can::Pair(a), Can::Pair(b)) => a.unify_with(&b, self),
                (Can::Vec(a), Can::Vec(b)) => a.unify_with(&b, self),
                _ => None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Can, State};
    use crate::LVar;
    use im::HashMap;

    #[test]
    fn new() {
        let state: State<u8> = State::new();
        assert_eq!(state.values, HashMap::new());
    }

    #[test]
    fn assign() {
        let state: State<u8> = State::new();
        let x = LVar::new();
        let state = state.assign(x, Can::Val(5));
        assert_eq!(state.values, HashMap::unit(x, Can::Val(5)));
    }

    #[test]
    fn value_of_direct() {
        let state: State<u8> = State::new();
        let x = LVar::new();
        let state = state.assign(x, Can::Val(5));
        assert_eq!(state.resolve_var(x), Can::Val(5));
    }

    #[test]
    fn value_of_missing() {
        let state: State<u8> = State::new();
        let x = LVar::new();
        assert_eq!(state.resolve_var(x), Can::Var(x));
        assert_eq!(state.resolve(&Can::Val(5)), Can::Val(5));
    }
    #[test]
    fn value_of_nested() {
        let state: State<u8> = State::new();
        let x = LVar::new();
        let y = LVar::new();
        let z = LVar::new();
        let state = state.assign(x, Can::Var(y));
        let state = state.assign(y, Can::Var(z));
        let state = state.assign(z, Can::Val(5));

        assert_eq!(state.resolve_var(x), Can::Val(5));
        assert_eq!(state.resolve_var(y), Can::Val(5));
        assert_eq!(state.resolve_var(z), Can::Val(5));
    }
    #[test]
    fn unify_with_self() {
        let state: State<u8> = State::new();
        let x = LVar::new();
        let unified = state.unify(&Can::Var(x), &Can::Var(x));
        assert_eq!(unified.unwrap(), state);
    }
    #[test]
    fn unify_two_vars() {
        let state: State<u8> = State::new();
        let x = LVar::new();
        let y = LVar::new();

        assert_eq!(
            state.unify(&Can::Var(x), &Can::Var(y)).unwrap(),
            state.assign(x, Can::Var(y))
        );
    }
    #[test]
    fn unify_with_value() {
        let x = LVar::new();
        let state: State<u8> = State::new();

        assert_eq!(
            state.unify(&Can::Var(x), &Can::Val(5)).unwrap(),
            state.assign(x, Can::Val(5))
        );
        assert_eq!(
            state.unify(&Can::Val(5), &Can::Var(x)).unwrap(),
            state.assign(x, Can::Val(5))
        );
    }
    #[test]
    fn unify_already_bound() {
        let x = LVar::new();
        let state: State<u8> = State::new().assign(x, Can::Val(5));
        assert_eq!(state.unify(&Can::Var(x), &Can::Val(6)), None);
    }

    #[test]
    fn unify_list() {
        let x = LVar::new();
        let state: State<u8> = State::new();
        let unified = state.unify(
            &Can::Vec(vec![Can::Val(1), Can::Var(x), Can::Val(3)]),
            &Can::Vec(vec![Can::Val(1), Can::Val(2), Can::Val(3)]),
        );
        assert_eq!(unified.unwrap().resolve_var(x), Can::Val(2));
    }
}
