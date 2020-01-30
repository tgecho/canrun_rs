use crate::can::lvar::LVar;
use crate::can::{Can, CanT};
use crate::goal::GoalIter;
use crate::unify::Unify;
use im::hashmap::HashMap;
use std::iter::{empty, once};
use std::rc::Rc;

#[derive(Clone, PartialEq, Eq, Debug)]
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
            Can::Pair { l, r } => Can::Pair {
                l: Box::new(self.resolve(l)),
                r: Box::new(self.resolve(r)),
            },
            Can::Vec(v) => {
                let resolved = v.iter().map(|i| self.resolve(i));
                Can::Vec(resolved.collect())
            }
            Can::Nil => Can::Nil,
            Can::Contains(c) => Can::Contains(Box::new(self.resolve(c))),
        }
    }

    pub fn resolve_var(&self, key: LVar) -> Can<T> {
        self.resolve(&Can::Var(key))
    }

    pub fn unify(&self, a: &Can<T>, b: &Can<T>) -> GoalIter<T> {
        let a = self.resolve(a);
        let b = self.resolve(b);

        if a == b {
            Box::new(once(self.clone())) as GoalIter<_>
        } else {
            match (a, b) {
                (Can::Var(av), bv) => Box::new(once(self.assign(av, bv))),
                (av, Can::Var(bv)) => Box::new(once(self.assign(bv, av))),
                (Can::Pair { l: al, r: ar }, Can::Pair { l: bl, r: br }) => {
                    Box::new(self.unify(&al, &bl).flat_map(move |l| l.unify(&ar, &*br)))
                }
                (Can::Vec(a), Can::Vec(b)) => {
                    if a.len() == b.len() {
                        let mut pairs = a.iter().zip(b.iter());
                        // Start with a single copy of the state
                        let initial = vec![self.clone()];
                        let states = pairs.try_fold(initial, |states, (a, b)| {
                            // Try to unify the two sides in each state and flatten out the results
                            // Failed unifications will return empty and those states will drop out
                            if states.len() > 0 {
                                Some(states.iter().flat_map(|s| s.unify(a, b)).collect())
                            } else {
                                None
                            }
                        });
                        match states {
                            Some(states) => Box::new(states.into_iter()),
                            None => Box::new(empty()),
                        }
                    } else {
                        Box::new(empty())
                    }
                }
                (Can::Contains(needle), Can::Vec(vec)) => {
                    unify_contains(*needle, vec, self.clone())
                }
                _ => Box::new(empty()),
            }
        }
    }
}

fn unify_contains<T: CanT + 'static>(
    needle: Can<T>,
    haystack: Vec<Can<T>>,
    state: State<T>,
) -> GoalIter<T> {
    Box::new(
        haystack
            .into_iter()
            .flat_map(move |c| state.unify(&needle, &c)),
    )
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

    #[test]
    fn unify_list() {
        let x = LVar::new();
        let state: State<u8> = State::new();
        let mut unified = state.unify(
            &Can::Vec(vec![Can::Val(1), Can::Var(x), Can::Val(3)]),
            &Can::Vec(vec![Can::Val(1), Can::Val(2), Can::Val(3)]),
        );
        assert_eq!(unified.nth(0).unwrap().resolve_var(x), Can::Val(2));
    }
}
