use crate::state;
use crate::util::multikeyvaluemap::Value as MultiMapValue;
use crate::{Can, CanT, Goal, LVar, State, StateIter};

#[derive(Clone, PartialEq, Debug)]
pub struct Constraint<T: CanT> {
    pub left: Can<T>,
    pub right: Can<T>,
    pub func: fn(T, T) -> bool,
}

impl<'a, T: CanT + 'a> Constraint<T> {
    pub fn run(self, state: State<'a, T>) -> StateIter<'a, T> {
        match (self.left.clone(), self.right.clone()) {
            (Can::Var(left), Can::Var(right)) => Box::new(
                state
                    .add_constraint(vec![left, right], self)
                    .check_constraints(left.can()),
            ),
            (Can::Var(left), _) => Box::new(
                state
                    .add_constraint(vec![left], self)
                    .check_constraints(left.can()),
            ),
            (_, Can::Var(right)) => Box::new(
                state
                    .add_constraint(vec![right], self)
                    .check_constraints(right.can()),
            ),
            (Can::Val(left), Can::Val(right)) => {
                if self.evaluate(left, right) {
                    state.to_iter()
                } else {
                    state::empty_iter()
                }
            }
            _ => state::empty_iter(),
        }
    }

    pub fn evaluate(&self, left: T, right: T) -> bool {
        let func = self.func;
        func(left, right)
    }
}

pub fn constrain<'a, T: CanT>(a: Can<T>, b: Can<T>, func: fn(T, T) -> bool) -> Goal<'a, T> {
    Goal::Constrain(Constraint {
        left: a,
        right: b,
        func,
    })
}

impl<'a, T: CanT + 'a> State<'a, T> {
    pub(crate) fn add_constraint(&self, vars: Vec<LVar>, constraint: Constraint<T>) -> Self {
        State {
            values: self.values.clone(),
            constraints: self.constraints.set(vars, constraint),
            mappings: self.mappings.clone(),
        }
    }

    pub(crate) fn add_constraint_key(
        &self,
        key: LVar,
        constraint: &MultiMapValue<LVar, Constraint<T>>,
    ) -> Self {
        State {
            values: self.values.clone(),
            constraints: self.constraints.add_key(key, constraint),
            mappings: self.mappings.clone(),
        }
    }

    pub(crate) fn remove_constraint(
        &self,
        constraint: &MultiMapValue<LVar, Constraint<T>>,
    ) -> Self {
        State {
            values: self.values.clone(),
            constraints: self.constraints.remove(constraint),
            mappings: self.mappings.clone(),
        }
    }

    pub(crate) fn check_constraints(self, can: Can<T>) -> StateIter<'a, T> {
        match can {
            Can::Var(lvar) => {
                let constraints = self.constraints.get(&lvar);
                let satisfied = constraints.iter().try_fold(self.clone(), |state, found| {
                    let constraint = &found.value;
                    match (
                        self.resolve(&constraint.left).ok()?,
                        self.resolve(&constraint.right).ok()?,
                    ) {
                        (Can::Val(left), Can::Val(right)) => {
                            if constraint.evaluate(left, right) {
                                Some(state.remove_constraint(found))
                            } else {
                                None
                            }
                        }
                        (Can::Var(left), _) => {
                            if left == lvar {
                                Some(state)
                            } else {
                                Some(state.add_constraint_key(left, found))
                            }
                        }
                        (_, Can::Var(right)) => {
                            if right == lvar {
                                Some(state)
                            } else {
                                Some(state.add_constraint_key(right, found))
                            }
                        }
                        _ => None,
                    }
                });
                match satisfied {
                    Some(state) => state.to_iter(),
                    None => state::empty_iter(),
                }
            }
            // Base is not an LVar. This depends on the correct base LVar being
            // maintained in the constraint store.
            _ => self.to_iter(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::util::test;
    use crate::{constrain, var, Can, Equals};

    #[test]
    fn should_succeed_one() {
        let (x, y) = (var(), var());
        let goals = vec![
            constrain(x.can(), y.can(), |x, y| x < y),
            x.equals(1),
            y.equals(2),
        ];
        let expected = vec![vec![Can::Val(1), Can::Val(2)]];
        test::all_permutations_resolve_to(goals, &vec![x, y], expected);
    }

    #[test]
    fn should_fail_one() {
        let (x, y) = (var(), var());
        let goals = vec![
            constrain(x.can(), y.can(), |x, y| x > y),
            x.equals(1),
            y.equals(2),
        ];
        let expected = vec![];
        test::all_permutations_resolve_to(goals, &vec![x, y], expected);
    }

    #[test]
    fn should_fail_with_multiple_constraints() {
        let (x, y) = (var(), var());
        let goals = vec![
            constrain(x.can(), y.can(), |x, y| x < y),
            constrain(x.can(), y.can(), |x, y| x > y),
            x.equals(1),
            y.equals(2),
        ];
        let expected = vec![];
        test::all_permutations_resolve_to(goals, &vec![x, y], expected);
    }

    #[test]
    fn should_fail_with_multi_stepped_vars() {
        let (x, y, z, w) = (var(), var(), var(), var());
        let goals = vec![
            constrain(x.can(), y.can(), |x, y| x > y),
            z.equals(1),
            w.equals(2),
            x.equals(z.can()),
            y.equals(w.can()),
        ];
        let expected = vec![];
        test::all_permutations_resolve_to(goals, &vec![x, y], expected);
    }

    #[test]
    fn should_succeed_with_multi_stepped_vars() {
        let (x, y, z, w) = (var(), var(), var(), var());
        let goals = vec![
            constrain(x.can(), y.can(), |x, y| x < y),
            z.equals(1),
            w.equals(2),
            x.equals(z.can()),
            y.equals(w.can()),
        ];
        let expected = vec![vec![Can::Val(1), Can::Val(2)]];
        test::all_permutations_resolve_to(goals, &vec![x, y], expected);
    }
}
