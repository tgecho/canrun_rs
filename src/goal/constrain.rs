use crate::state;
use crate::{Can, CanT, Goal, State, StateIter};

#[derive(Clone, PartialEq, Debug)]
pub struct Constraint<T: CanT> {
    pub left: Can<T>,
    pub right: Can<T>,
    pub func: fn(T, T) -> bool,
}

impl<'a, T: CanT + 'a> Constraint<T> {
    pub fn run(self, state: State<T>) -> StateIter<'a, T> {
        match (self.left.clone(), self.right.clone()) {
            (Can::Var(left), _) => Box::new(
                state
                    .add_constraint(left, self)
                    .check_constraint(left.can()),
            ),
            (_, Can::Var(right)) => Box::new(
                state
                    .add_constraint(right, self)
                    .check_constraint(right.can()),
            ),
            (Can::Val(left), Can::Val(right)) => Box::new(self.evaluate(left, right).run(state)),
            _ => state::empty_iter(),
        }
    }

    pub fn evaluate(self, left: T, right: T) -> Goal<'a, T> {
        let func = self.func;
        if func(left, right) {
            Goal::Succeed
        } else {
            Goal::Fail
        }
    }
}

pub fn constrain<'a, T: CanT>(a: Can<T>, b: Can<T>, func: fn(T, T) -> bool) -> Goal<'a, T> {
    Goal::Constrain(Constraint {
        left: a,
        right: b,
        func,
    })
}

#[cfg(test)]
mod tests {
    use crate::{all, Can, CanT, Goal, LVar, State};
    use galvanic_test::test_suite;
    use itertools::Itertools;
    use std::iter::once;

    fn all_permutations<'a, T: CanT + 'a>(
        goals: Vec<Goal<'a, T>>,
        vars: Vec<LVar>,
    ) -> impl Iterator<Item = (Vec<Goal<'a, T>>, Vec<LVar>)> + 'a {
        let goals_len = goals.len();
        goals
            .into_iter()
            .permutations(goals_len)
            .zip(once(vars).cycle())
    }

    fn resolve_to<'a, T: CanT + 'a>(
        goals: &Vec<Goal<'a, T>>,
        vars: &Vec<LVar>,
    ) -> Vec<Vec<Can<T>>> {
        all(goals.clone())
            .run(State::new())
            .map(|s| {
                let results = vars.iter().map(|v| s.resolve_var(*v).unwrap());
                results.collect::<Vec<Can<T>>>()
            })
            .collect()
    }

    test_suite! {
        use crate::{constrain, Can, Equals, var,Goal, LVar};
        use super::{all_permutations, resolve_to};

        fixture success_cases(goals: Vec<Goal<'static, usize>>, vars: Vec<LVar>) -> Vec<Vec<Can<usize>>> {
            params {
                let (x, y) = (var(), var());
                let goals = vec![
                    constrain(x.can(), y.can(), |x, y| x < y),
                    x.equals(1),
                    y.equals(2),
                ];
                all_permutations(goals, vec![x, y])
            }
            setup(&mut self) {
                resolve_to(self.goals, self.vars)
            }
        }

        test success(success_cases) {
            assert_eq!(success_cases.val, vec![vec![Can::Val(1), Can::Val(2)]])
        }

        fixture fail_cases(goals: Vec<Goal<'static, usize>>, vars: Vec<LVar>) -> Vec<Vec<Can<usize>>> {
            params {
                let (x, y) = (var(), var());
                let goals = vec![
                    constrain(x.can(), y.can(), |x, y| x > y),
                    x.equals(1),
                    y.equals(2),
                ];
                all_permutations(goals, vec![x, y])
            }
            setup(&mut self) {
                resolve_to(self.goals, self.vars)
            }
        }

        test fail(fail_cases) {
            assert!(fail_cases.val.is_empty())
        }

        fixture multiple_constraints_cases(goals: Vec<Goal<'static, usize>>, vars: Vec<LVar>) -> Vec<Vec<Can<usize>>> {
            params {
                let (x, y) = (var(), var());
                let goals = vec![
                    constrain(x.can(), y.can(), |x, y| x < y),
                    constrain(x.can(), y.can(), |x, y| x > y),
                    x.equals(1),
                    y.equals(2),
                ];
                all_permutations(goals, vec![x, y])
            }
            setup(&mut self) {
                resolve_to(self.goals, self.vars)
            }
        }

        test multiple_constraints(multiple_constraints_cases) {
            assert!(multiple_constraints_cases.val.is_empty())
        }
    }
}
