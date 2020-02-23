use crate::goal::custom::custom;
use crate::state;
use crate::{Can, CanT, Goal, LVar, State, StateIter};
use std::fmt;
use std::iter::once;
use std::rc::Rc;

type ConstraintResult<'a, T> = Result<Goal<'a, T>, Vec<LVar>>;

#[derive(Clone)]
pub enum Constraint<'a, T: CanT> {
    One {
        a: Can<T>,
        func: Rc<dyn Fn(Can<T>) -> ConstraintResult<'a, T> + 'a>,
    },
    Two {
        a: Can<T>,
        b: Can<T>,
        func: Rc<dyn Fn(Can<T>, Can<T>) -> ConstraintResult<'a, T> + 'a>,
    },
    // Three {
    //     a: Can<T>,
    //     b: Can<T>,
    //     c: Can<T>,
    //     func: Rc<dyn Fn(Can<T>, Can<T>, Can<T>) -> ConstraintResult<'a, T> + 'a>,
    // },
}

impl<'a, T: CanT + 'a> Constraint<'a, T> {
    pub(crate) fn run(self, state: State<'a, T>) -> StateIter<'a, T> {
        match self.try_run(state) {
            Ok(iter) => Box::new(iter),
            Err(_) => state::empty_iter(),
        }
    }
    fn try_run(self, state: State<'a, T>) -> state::UnifyResult<'a, T> {
        let result = match &self {
            Constraint::One { a, func } => {
                let a = state.resolve(&a)?;
                func(a)
            }
            Constraint::Two { a, b, func } => {
                let a = state.resolve(&a)?;
                let b = state.resolve(&b)?;
                func(a, b)
            }
        };
        match result {
            Ok(goal) => Ok(goal.run(state)),
            Err(vars) => Ok(state.add_constraint(vars, self).to_iter()),
        }
    }
}

impl<'a, T: CanT + 'a> State<'a, T> {
    pub(crate) fn add_constraint(self, vars: Vec<LVar>, constraint: Constraint<'a, T>) -> Self {
        State {
            values: self.values.clone(),
            constraints: self.constraints.add(vars, constraint),
        }
    }

    pub(crate) fn check_constraints(self, lvar: LVar) -> StateIter<'a, T> {
        let (remaining_constraints, found_constraints) = self.constraints.extract(&lvar);
        let state = State {
            values: self.values.clone(),
            constraints: remaining_constraints,
        };
        found_constraints
            .into_iter()
            .fold(state.to_iter(), |state_iter, constraint| {
                Box::new(
                    state_iter
                        .zip(once(constraint).cycle())
                        .flat_map(|(s, c)| c.run(s)),
                )
            })
    }
}

impl<'a, T: CanT> fmt::Debug for Constraint<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Constraint::One { a, .. } => write!(f, "Constraint::One {:?}", a),
            Constraint::Two { a, b, .. } => write!(f, "Constraint::Two {:?} {:?}", a, b),
        }
    }
}

pub fn constrain<'a, T: CanT + 'a>(constraint: Constraint<'a, T>) -> Goal<'a, T> {
    custom(move |state| constraint.clone().run(state))
}

pub fn constrain_1<'a, T, F>(a: Can<T>, func: F) -> Goal<'a, T>
where
    T: CanT + 'a,
    F: Fn(T) -> bool + 'a,
{
    constrain(Constraint::One {
        a,
        func: Rc::new(move |a| match a {
            Can::Var(a) => Err(vec![a]),
            Can::Val(a) => Ok(if func(a) { Goal::Succeed } else { Goal::Fail }),
            _ => Ok(Goal::Fail),
        }),
    })
}

#[cfg(test)]
mod tests {
    use super::constrain_1;
    use crate::util::test;
    use crate::{var, Can, Equals};

    #[test]
    fn should_succeed_one() {
        let x = var();
        let goals = vec![x.equals(2), constrain_1(x.can(), |x| x > 1)];
        test::all_permutations_resolve_to(goals, &vec![x], vec![vec![Can::Val(2)]]);
    }

    #[test]
    fn should_fail_one() {
        let x = var();
        let goals = vec![x.equals(1), constrain_1(x.can(), |x| x > 1)];
        test::all_permutations_resolve_to(goals, &vec![x], vec![]);
    }
}
