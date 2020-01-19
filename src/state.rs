use crate::lvar::LVar;
use im::hashmap::HashMap;

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
pub enum Cell<T: Eq + Clone> {
    LVar(LVar),
    Value(T),
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct State<T: Eq + Clone> {
    values: HashMap<LVar, Cell<T>>,
}

impl<T: Eq + Clone> State<T> {
    pub fn new() -> State<T> {
        State {
            values: HashMap::new(),
        }
    }

    pub fn assign(&self, key: LVar, value: Cell<T>) -> Self {
        State {
            values: self.values.update(key, value),
        }
    }

    pub fn resolve(&self, key: &Cell<T>) -> Cell<T> {
        match key {
            Cell::LVar(lvar) => match self.values.get(lvar) {
                Some(val) => self.resolve(val),
                None => key.clone(),
            },
            Cell::Value(_) => key.clone(),
        }
    }

    pub fn resolve_var(&self, key: LVar) -> Cell<T> {
        self.resolve(&Cell::LVar(key))
    }

    pub fn unify(&self, a: &Cell<T>, b: &Cell<T>) -> Option<State<T>> {
        let a = self.resolve(a);
        let b = self.resolve(b);

        if a == b {
            Some(self.clone())
        } else {
            match (a, b) {
                (Cell::LVar(av), bv) => Some(self.assign(av, bv)),
                (av, Cell::LVar(bv)) => Some(self.assign(bv, av)),
                _ => None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Cell, State};
    use crate::lvar::LVar;
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
        let state = state.assign(x, Cell::Value(5));
        assert_eq!(state.values, HashMap::unit(x, Cell::Value(5)));
    }

    #[test]
    fn value_of_direct() {
        let state: State<u8> = State::new();
        let x = LVar::new();
        let state = state.assign(x, Cell::Value(5));
        assert_eq!(state.resolve_var(x), Cell::Value(5));
    }

    #[test]
    fn value_of_missing() {
        let state: State<u8> = State::new();
        let x = LVar::new();
        assert_eq!(state.resolve_var(x), Cell::LVar(x));
        assert_eq!(state.resolve(&Cell::Value(5)), Cell::Value(5));
    }
    #[test]
    fn value_of_nested() {
        let state: State<u8> = State::new();
        let x = LVar::new();
        let y = LVar::new();
        let z = LVar::new();
        let state = state.assign(x, Cell::LVar(y));
        let state = state.assign(y, Cell::LVar(z));
        let state = state.assign(z, Cell::Value(5));

        assert_eq!(state.resolve_var(x), Cell::Value(5));
        assert_eq!(state.resolve_var(y), Cell::Value(5));
        assert_eq!(state.resolve_var(z), Cell::Value(5));
    }
    #[test]
    fn unify_with_self() {
        let state: State<u8> = State::new();
        let x = LVar::new();
        let unified = state.unify(&Cell::LVar(x), &Cell::LVar(x));
        assert_eq!(unified.unwrap(), state);
    }
    #[test]
    fn unify_two_vars() {
        let state: State<u8> = State::new();
        let x = LVar::new();
        let y = LVar::new();

        assert_eq!(
            state.unify(&Cell::LVar(x), &Cell::LVar(y)).unwrap(),
            state.assign(x, Cell::LVar(y))
        );
    }
    #[test]
    fn unify_with_value() {
        let x = LVar::new();
        let state: State<u8> = State::new();

        assert_eq!(
            state.unify(&Cell::LVar(x), &Cell::Value(5)).unwrap(),
            state.assign(x, Cell::Value(5))
        );
        assert_eq!(
            state.unify(&Cell::Value(5), &Cell::LVar(x)).unwrap(),
            state.assign(x, Cell::Value(5))
        );
    }
    #[test]
    fn unify_already_bound() {
        let x = LVar::new();
        let state: State<u8> = State::new().assign(x, Cell::Value(5));
        assert_eq!(state.unify(&Cell::LVar(x), &Cell::Value(6)), None);
    }
}
