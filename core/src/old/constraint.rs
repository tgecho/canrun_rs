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
    Three {
        a: Can<T>,
        b: Can<T>,
        c: Can<T>,
        func: Rc<dyn Fn(Can<T>, Can<T>, Can<T>) -> ConstraintResult<'a, T> + 'a>,
    },
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
            Constraint::Three { a, b, c, func } => {
                let a = state.resolve(&a)?;
                let b = state.resolve(&b)?;
                let c = state.resolve(&c)?;
                func(a, b, c)
            }
        };
        match result {
            Ok(goal) => Ok(goal.run(state)),
            Err(vars) => Ok(state.add_constraint(vars, self).to_iter()),
        }
    }

    pub(crate) fn one<F>(a: Can<T>, func: F) -> Constraint<'a, T>
    where
        F: Fn(Can<T>) -> ConstraintResult<'a, T> + 'a,
    {
        Constraint::One {
            a,
            func: Rc::new(func),
        }
    }
    pub(crate) fn two<F>(a: Can<T>, b: Can<T>, func: F) -> Constraint<'a, T>
    where
        F: Fn(Can<T>, Can<T>) -> ConstraintResult<'a, T> + 'a,
    {
        Constraint::Two {
            a,
            b,
            func: Rc::new(func),
        }
    }
    pub(crate) fn three<F>(a: Can<T>, b: Can<T>, c: Can<T>, func: F) -> Constraint<'a, T>
    where
        F: Fn(Can<T>, Can<T>, Can<T>) -> ConstraintResult<'a, T> + 'a,
    {
        Constraint::Three {
            a,
            b,
            c,
            func: Rc::new(func),
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
            Constraint::Three { a, b, c, .. } => {
                write!(f, "Constraint::Three {:?} {:?} {:?}", a, b, c)
            }
        }
    }
}

pub fn constrain<'a, T: CanT + 'a>(constraint: Constraint<'a, T>) -> Goal<'a, T> {
    custom(move |state| constraint.clone().run(state))
}
