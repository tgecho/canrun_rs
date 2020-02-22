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

#[cfg(test)]
mod tests {
    use crate::{all, constrain, var, Can, CanT, Equals, Goal, LVar, State};
    use itertools::Itertools;

    fn all_permutations<'a, T: CanT + 'a>(
        goals: Vec<Goal<'a, T>>,
    ) -> impl Iterator<Item = Vec<Goal<'a, T>>> + 'a {
        let goals_len = goals.len();
        goals.into_iter().permutations(goals_len)
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

    #[test]
    fn should_succeed_one() {
        let (x, y) = (var(), var());
        let goals = vec![
            constrain(x.can(), y.can(), |x, y| x < y),
            x.equals(1),
            y.equals(2),
        ];
        for goals in all_permutations(goals) {
            let resolved = resolve_to(&goals, &vec![x, y]);
            dbg!(goals);
            assert_eq!(resolved, vec![vec![Can::Val(1), Can::Val(2)]]);
        }
    }

    #[test]
    fn should_fail_one() {
        let (x, y) = (var(), var());
        let goals = vec![
            constrain(x.can(), y.can(), |x, y| x > y),
            x.equals(1),
            y.equals(2),
        ];
        for goals in all_permutations(goals) {
            let resolved = resolve_to(&goals, &vec![x, y]);
            dbg!(goals);
            assert!(resolved.is_empty());
        }
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
        for goals in all_permutations(goals) {
            let resolved = resolve_to(&goals, &vec![x, y]);
            dbg!(goals);
            assert!(resolved.is_empty());
        }
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
        for goals in all_permutations(goals) {
            let resolved = resolve_to(&goals, &vec![x, y]);
            dbg!(goals);
            assert!(resolved.is_empty());
            println!("^ passes ^\n");
        }
    }
}
