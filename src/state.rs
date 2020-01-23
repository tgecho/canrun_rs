use crate::lvar::LVar;
use im::hashmap::HashMap;

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
pub enum Cell<T: Eq + Clone> {
    Var(LVar),
    Value(T),
    Pair(Box<(Cell<T>, Cell<T>)>),
    List(Vec<Cell<T>>),
}

pub fn pair<T: Eq + Clone>(a: Cell<T>, b: Cell<T>) -> Cell<T> {
    Cell::Pair(Box::new((a, b)))
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
            Cell::Var(lvar) => match self.values.get(lvar) {
                Some(val) => self.resolve(val),
                None => key.clone(),
            },
            Cell::Value(_) => key.clone(),
            Cell::Pair(pair) => {
                Cell::Pair(Box::new((self.resolve(&pair.0), self.resolve(&pair.1))))
            }
            Cell::List(list) => {
                let resolved = list.iter().map(|i| self.resolve(i));
                Cell::List(resolved.collect())
            }
        }
    }

    pub fn resolve_var(&self, key: LVar) -> Cell<T> {
        self.resolve(&Cell::Var(key))
    }

    pub fn unify(&self, a: &Cell<T>, b: &Cell<T>) -> Option<State<T>> {
        let a = self.resolve(a);
        let b = self.resolve(b);

        if a == b {
            Some(self.clone())
        } else {
            match (a, b) {
                (Cell::Var(av), bv) => Some(self.assign(av, bv)),
                (av, Cell::Var(bv)) => Some(self.assign(bv, av)),
                (Cell::Pair(a), Cell::Pair(b)) => self
                    .unify(&a.0, &b.0)
                    .and_then(|state| state.unify(&a.1, &b.1)),
                (Cell::List(a), Cell::List(b)) => {
                    if a.len() == b.len() {
                        let initial = self.clone();
                        let mut pairs = a.iter().zip(b.iter());
                        pairs.try_fold(initial, |s, (a, b)| s.unify(a, b))
                    } else {
                        None
                    }
                }
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
        assert_eq!(state.resolve_var(x), Cell::Var(x));
        assert_eq!(state.resolve(&Cell::Value(5)), Cell::Value(5));
    }
    #[test]
    fn value_of_nested() {
        let state: State<u8> = State::new();
        let x = LVar::new();
        let y = LVar::new();
        let z = LVar::new();
        let state = state.assign(x, Cell::Var(y));
        let state = state.assign(y, Cell::Var(z));
        let state = state.assign(z, Cell::Value(5));

        assert_eq!(state.resolve_var(x), Cell::Value(5));
        assert_eq!(state.resolve_var(y), Cell::Value(5));
        assert_eq!(state.resolve_var(z), Cell::Value(5));
    }
    #[test]
    fn unify_with_self() {
        let state: State<u8> = State::new();
        let x = LVar::new();
        let unified = state.unify(&Cell::Var(x), &Cell::Var(x));
        assert_eq!(unified.unwrap(), state);
    }
    #[test]
    fn unify_two_vars() {
        let state: State<u8> = State::new();
        let x = LVar::new();
        let y = LVar::new();

        assert_eq!(
            state.unify(&Cell::Var(x), &Cell::Var(y)).unwrap(),
            state.assign(x, Cell::Var(y))
        );
    }
    #[test]
    fn unify_with_value() {
        let x = LVar::new();
        let state: State<u8> = State::new();

        assert_eq!(
            state.unify(&Cell::Var(x), &Cell::Value(5)).unwrap(),
            state.assign(x, Cell::Value(5))
        );
        assert_eq!(
            state.unify(&Cell::Value(5), &Cell::Var(x)).unwrap(),
            state.assign(x, Cell::Value(5))
        );
    }
    #[test]
    fn unify_already_bound() {
        let x = LVar::new();
        let state: State<u8> = State::new().assign(x, Cell::Value(5));
        assert_eq!(state.unify(&Cell::Var(x), &Cell::Value(6)), None);
    }

    #[test]
    fn unify_list() {
        let x = LVar::new();
        let state: State<u8> = State::new();
        let unified = state.unify(
            &Cell::List(vec![Cell::Value(1), Cell::Var(x), Cell::Value(3)]),
            &Cell::List(vec![Cell::Value(1), Cell::Value(2), Cell::Value(3)]),
        );
        assert_eq!(unified.unwrap().resolve_var(x), Cell::Value(2));
    }
}
