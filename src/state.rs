use crate::can::lvar::LVar;
use crate::can::{pair, vec, Can, CanT};
use crate::goal::StateIter;
use im::hashmap::HashMap;
use std::iter::{empty, once};

#[derive(Clone, PartialEq, Debug)]
pub struct State<T: CanT> {
    values: HashMap<LVar, Can<T>>,
}

impl<T: CanT + 'static> State<T> {
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
            Can::Pair { l, r } => pair::resolve(self, l, r),
            Can::Vec(v) => vec::resolve(self, v),
            Can::Nil => Can::Nil,
            Can::HoC { value, unify } => Can::HoC {
                value: Box::new(self.resolve(value)),
                unify: *unify,
            },
        }
    }

    pub fn resolve_var(&self, key: LVar) -> Can<T> {
        self.resolve(&Can::Var(key))
    }

    pub fn unify(&self, a: &Can<T>, b: &Can<T>) -> StateIter<T> {
        let a = self.resolve(a);
        let b = self.resolve(b);

        if a == b {
            Box::new(once(self.clone())) as StateIter<T>
        } else {
            match (a, b) {
                (Can::Var(av), bv) => Box::new(once(self.assign(av, bv))),
                (av, Can::Var(bv)) => Box::new(once(self.assign(bv, av))),
                (Can::Pair { l: al, r: ar }, Can::Pair { l: bl, r: br }) => {
                    pair::unify(self, *al, *ar, *bl, *br)
                }
                (Can::Vec(a), Can::Vec(b)) => vec::unify(self, a, b),
                (Can::HoC { value, unify }, other) => unify(*value, other, self.clone()),
                _ => Box::new(empty()),
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
        let unified = state.unify(&Can::Var(x), &Can::Var(x)).nth(0);
        assert_eq!(unified.unwrap(), state);
    }
    #[test]
    fn unify_two_vars() {
        let state: State<u8> = State::new();
        let x = LVar::new();
        let y = LVar::new();

        assert_eq!(
            state.unify(&Can::Var(x), &Can::Var(y)).nth(0).unwrap(),
            state.assign(x, Can::Var(y))
        );
    }
    #[test]
    fn unify_with_value() {
        let x = LVar::new();
        let state: State<u8> = State::new();

        assert_eq!(
            state.unify(&Can::Var(x), &Can::Val(5)).nth(0).unwrap(),
            state.assign(x, Can::Val(5))
        );
        assert_eq!(
            state.unify(&Can::Val(5), &Can::Var(x)).nth(0).unwrap(),
            state.assign(x, Can::Val(5))
        );
    }
    #[test]
    fn unify_already_bound() {
        let x = LVar::new();
        let state: State<u8> = State::new().assign(x, Can::Val(5));
        let result: Vec<_> = state.unify(&Can::Var(x), &Can::Val(6)).collect();
        assert_eq!(result, vec![]);
    }
}
