use crate::can::lvar::LVar;
use crate::can::{pair, vec, Can, CanT};
use crate::goal::StateIter;
use im::{HashMap, HashSet};
use std::iter::{empty, once};

#[derive(Clone, PartialEq, Debug)]
pub struct Constraint<T: CanT> {
    pub left: LVar,
    pub right: LVar,
    pub func: fn(T, T) -> bool,
}

#[derive(Clone, PartialEq, Debug, Default)]
pub struct State<T: CanT> {
    values: HashMap<LVar, Can<T>>,
    constraints: HashMap<LVar, Constraint<T>>,
}

impl<'a, T: CanT + 'a> State<T> {
    pub fn new() -> State<T> {
        State {
            values: HashMap::new(),
            constraints: HashMap::new(),
        }
    }

    pub(crate) fn assign(&self, var: LVar, value: Can<T>) -> Self {
        State {
            values: self.values.update(var, value),
            constraints: self.constraints.clone(),
        }
    }

    pub(crate) fn constrain(&self, constraint: Constraint<T>) -> Option<Self> {
        let anchor = constraint.left;
        let constrained = State {
            values: self.values.clone(),
            constraints: self.constraints.update(anchor, constraint),
        };
        constrained.check_constraint(anchor)
    }

    pub(crate) fn checked_resolve(
        &self,
        can: &Can<T>,
        history: &HashSet<LVar>,
    ) -> ResolveResult<T> {
        match can {
            Can::Var(lvar) => {
                if history.contains(lvar) {
                    debug!("{:?}", history);
                    Err(UnifyError::InfiniteRecursion(*lvar))
                } else {
                    let history = history.update(*lvar);
                    match self.values.get(lvar) {
                        Some(val) => self.checked_resolve(val, &history),
                        None => Ok(can.clone()),
                    }
                }
            }
            Can::Val(v) => Ok(Can::Val(v.clone())),
            Can::Pair { l, r } => pair::resolve(self, l, r, history),
            Can::Vec(v) => vec::resolve(self, v, history),
            Can::Nil => Ok(Can::Nil),
            Can::Hoc(hoc) => hoc.resolve_in(self, history),
        }
    }

    pub fn resolve(&self, can: &Can<T>) -> ResolveResult<T> {
        self.checked_resolve(can, &HashSet::new())
    }

    pub fn resolve_var(&self, key: LVar) -> ResolveResult<T> {
        self.resolve(&Can::Var(key))
    }

    pub fn unify(self, a: Can<T>, b: Can<T>) -> StateIter<'a, T> {
        self.try_unify(a, b).unwrap_or_else(|err| {
            debug!("{:?}", err);
            Box::new(empty())
        })
    }

    pub(crate) fn check_constraint(self, lvar: LVar) -> Option<Self> {
        match self.constraints.extract(&lvar) {
            Some((constraint, constraints)) => match self.resolve(&constraint.left.can()).ok()? {
                Can::Var(left) if left == lvar => Some(self),
                Can::Var(left) => Some(State {
                    values: self.values.clone(),
                    constraints: constraints.update(left, constraint),
                }),
                Can::Val(left) => match self.resolve(&constraint.right.can()).ok()? {
                    Can::Var(right) if right == lvar => Some(self),
                    Can::Var(right) => Some(State {
                        values: self.values.clone(),
                        constraints: constraints.update(right, constraint),
                    }),
                    Can::Val(right) => {
                        let func = constraint.func;
                        if func(left, right) {
                            Some(State {
                                values: self.values.clone(),
                                constraints: constraints,
                            })
                        } else {
                            None
                        }
                    }
                    _ => None,
                },
                _ => None,
            },
            None => Some(self),
        }
    }

    fn try_unify(self, a_: Can<T>, b_: Can<T>) -> UnifyResult<'a, T> {
        let a = self.resolve(&a_)?;
        let b = self.resolve(&b_)?;

        Ok(if a == b {
            Box::new(once(self.clone())) as StateIter<T>
        } else {
            match (a, b) {
                (Can::Var(av), bv) => {
                    Box::new(self.assign(av, bv).check_constraint(av).into_iter())
                }
                (av, Can::Var(bv)) => {
                    Box::new(self.assign(bv, av).check_constraint(bv).into_iter())
                }
                (Can::Pair { l: al, r: ar }, Can::Pair { l: bl, r: br }) => {
                    pair::unify(self, *al, *ar, *bl, *br)
                }
                (Can::Vec(a), Can::Vec(b)) => vec::unify(self, a, b),
                (Can::Hoc(a), b) => a.unify_with(b, self),
                (a, Can::Hoc(b)) => b.unify_with(a, self),
                _ => Box::new(empty()),
            }
        })
    }
}

#[derive(Debug)]
pub enum UnifyError {
    InfiniteRecursion(LVar),
}

pub type ResolveResult<T> = Result<Can<T>, UnifyError>;
pub type UnifyResult<'a, T> = Result<StateIter<'a, T>, UnifyError>;

#[cfg(test)]
mod tests {
    use crate::{var, Can, LVar, State};
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
        assert_eq!(state.resolve_var(x).unwrap(), Can::Val(5));
    }

    #[test]
    fn value_of_missing() {
        let state: State<u8> = State::new();
        let x = var();
        assert_eq!(state.resolve_var(x).unwrap(), x.can());
        assert_eq!(state.resolve(&Can::Val(5)).unwrap(), Can::Val(5));
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

        assert_eq!(state.resolve_var(x).unwrap(), Can::Val(5));
        assert_eq!(state.resolve_var(y).unwrap(), Can::Val(5));
        assert_eq!(state.resolve_var(z).unwrap(), Can::Val(5));
    }
    #[test]
    fn unify_with_self() {
        let state: State<u8> = State::new();
        let x = var();
        let unified = state.clone().unify(x.can(), x.can()).nth(0);
        assert_eq!(unified.unwrap(), state);
    }
    #[test]
    fn unify_two_vars() {
        let state: State<u8> = State::new();
        let x = LVar::new();
        let y = LVar::new();

        assert_eq!(
            state
                .clone()
                .unify(Can::Var(x), Can::Var(y))
                .nth(0)
                .unwrap(),
            state.assign(x, Can::Var(y))
        );
    }
    #[test]
    fn unify_with_value() {
        let x = LVar::new();
        let state: State<u8> = State::new();

        assert_eq!(
            (state.clone())
                .unify(Can::Var(x), Can::Val(5))
                .nth(0)
                .unwrap(),
            state.assign(x, Can::Val(5))
        );
        assert_eq!(
            (state.clone())
                .unify(Can::Val(5), Can::Var(x))
                .nth(0)
                .unwrap(),
            state.assign(x, Can::Val(5))
        );
    }
    #[test]
    fn unify_already_bound() {
        let x = LVar::new();
        let state: State<u8> = State::new().assign(x, Can::Val(5));
        let result: Vec<_> = state.unify(Can::Var(x), Can::Val(6)).collect();
        assert_eq!(result, vec![]);
    }
}
