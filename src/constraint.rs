use crate::goal::custom::custom;
use crate::state;
use crate::{
    equal, Can,
    Can::{Val, Var},
    CanT, Goal, LVar, State, StateIter,
};
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

pub fn constrain_1<'a, T, F>(a: Can<T>, func: F) -> Goal<'a, T>
where
    T: CanT + 'a,
    F: Fn(T) -> bool + 'a,
{
    constrain(Constraint::One {
        a,
        func: Rc::new(move |a| match a {
            Var(a) => Err(vec![a]),
            Val(a) => Ok(if func(a) { Goal::Succeed } else { Goal::Fail }),
            _ => Ok(Goal::Fail),
        }),
    })
}

pub fn constrain_2<'a, T, F>(a: Can<T>, b: Can<T>, func: F) -> Goal<'a, T>
where
    T: CanT + 'a,
    F: Fn(T, T) -> bool + 'a,
{
    constrain(Constraint::Two {
        a,
        b,
        func: Rc::new(move |a, b| match (a, b) {
            (Val(a), Val(b)) => Ok(if func(a, b) {
                Goal::Succeed
            } else {
                Goal::Fail
            }),
            (Var(a), Var(b)) => Err(vec![a, b]),
            (Var(a), _) => Err(vec![a]),
            (_, Var(b)) => Err(vec![b]),
            _ => Ok(Goal::Fail),
        }),
    })
}

pub fn map_2<'a, T, AB, BA>(a: Can<T>, b: Can<T>, ab: AB, ba: BA) -> Goal<'a, T>
where
    T: CanT + 'a,
    AB: Fn(T) -> T + 'a,
    BA: Fn(T) -> T + 'a,
{
    constrain(Constraint::Two {
        a,
        b,
        func: Rc::new(move |a, b| match (a, b) {
            (Val(a), b) => Ok(equal(ab(a), b)),
            (a, Val(b)) => Ok(equal(a, ba(b))),
            (Var(a), Var(b)) => Err(vec![a, b]),
            _ => Ok(Goal::Fail),
        }),
    })
}

pub fn map_3<'a, T, AB, BC, AC>(
    a: Can<T>,
    b: Can<T>,
    c: Can<T>,
    ab: AB,
    bc: BC,
    ac: AC,
) -> Goal<'a, T>
where
    T: CanT + 'a,
    AB: Fn(T, T) -> T + 'a,
    BC: Fn(T, T) -> T + 'a,
    AC: Fn(T, T) -> T + 'a,
{
    constrain(Constraint::Three {
        a,
        b,
        c,
        func: Rc::new(move |a, b, c| match (a, b, c) {
            (Val(a), Val(b), c) => Ok(equal(ab(a, b), c)),
            (a, Val(b), Val(c)) => Ok(equal(a, bc(b, c))),
            (Val(a), b, Val(c)) => Ok(equal(ac(a, c), b)),
            (Var(a), Var(b), Var(c)) => Err(vec![a, b, c]),
            (Var(a), Var(b), _) => Err(vec![a, b]),
            (_, Var(b), Var(c)) => Err(vec![b, c]),
            (Var(a), _, Var(c)) => Err(vec![a, c]),
            _ => Ok(Goal::Fail),
        }),
    })
}

#[cfg(test)]
mod tests {
    use super::{constrain_1, constrain_2, map_2, map_3};
    use crate::util::test;
    use crate::{var, Can, Equals, Goal};

    #[test]
    fn should_succeed_constrain_1() {
        let x = var();
        let goals = vec![x.equals(2), constrain_1(x.can(), |x| x > 1)];
        test::all_permutations_resolve_to(goals, &vec![x], vec![vec![Can::Val(2)]]);
    }

    #[test]
    fn should_fail_constrain_1() {
        let x = var();
        let goals = vec![x.equals(1), constrain_1(x.can(), |x| x > 1)];
        test::all_permutations_resolve_to(goals, &vec![x], vec![]);
    }
    #[test]
    fn should_succeed_constrain_2() {
        let (x, y) = (var(), var());
        let goals = vec![
            x.equals(1),
            y.equals(2),
            constrain_2(x.can(), y.can(), |x, y| x < y),
        ];
        test::all_permutations_resolve_to(goals, &vec![x, y], vec![vec![Can::Val(1), Can::Val(2)]]);
    }
    #[test]
    fn should_fail_constrain_2() {
        let (x, y) = (var(), var());
        let goals = vec![
            x.equals(1),
            y.equals(2),
            constrain_2(x.can(), y.can(), |x, y| x > y),
        ];
        test::all_permutations_resolve_to(goals, &vec![x, y], vec![]);
    }

    fn incr(n: usize) -> usize {
        n + 1
    }
    fn decr(n: usize) -> usize {
        n - 1
    }
    #[test]
    fn should_succeed_map_2() {
        let (x, y) = (var(), var());
        let expected = vec![vec![Can::Val(1), Can::Val(2)]];

        let goals = vec![
            x.equals(1),
            y.equals(2),
            map_2(x.can(), y.can(), incr, decr),
        ];
        test::all_permutations_resolve_to(goals, &vec![x, y], expected.clone());

        let goals = vec![x.equals(1), map_2(x.can(), y.can(), incr, decr)];
        test::all_permutations_resolve_to(goals, &vec![x, y], expected.clone());

        let goals = vec![y.equals(2), map_2(x.can(), y.can(), incr, decr)];
        test::all_permutations_resolve_to(goals, &vec![x, y], expected);
    }
    #[test]
    fn should_fail_map_2() {
        let (x, y) = (var(), var());
        let goals = vec![
            x.equals(2),
            y.equals(1),
            map_2(x.can(), y.can(), incr, decr),
        ];
        test::all_permutations_resolve_to(goals, &vec![x, y], vec![]);
    }

    fn add<'a>(a: Can<usize>, b: Can<usize>, c: Can<usize>) -> Goal<'a, usize> {
        map_3(a, b, c, |a, b| a + b, |a, c| c - a, |b, c| c - b)
    }

    #[test]
    fn should_succeed_add() {
        let (x, y, z) = (var(), var(), var());
        let scenarios = vec![
            vec![
                add(x.can(), y.can(), z.can()),
                x.equals(1),
                y.equals(2),
                z.equals(3),
            ],
            vec![add(x.can(), y.can(), z.can()), y.equals(2), z.equals(3)],
            vec![add(x.can(), y.can(), z.can()), x.equals(1), z.equals(3)],
            vec![add(x.can(), y.can(), z.can()), x.equals(1), y.equals(2)],
        ];
        for goals in scenarios {
            let expected = vec![vec![Can::Val(1), Can::Val(2), Can::Val(3)]];
            test::all_permutations_resolve_to(goals, &vec![x, y, z], expected);
        }
    }
}
