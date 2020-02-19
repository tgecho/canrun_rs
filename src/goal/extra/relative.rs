use crate::{constrain, Can, CanT, Goal, LVar};

pub fn greater_than<'a, T: CanT + PartialOrd>(a: Can<T>, b: Can<T>) -> Goal<'a, T> {
    constrain(a, b, |a, b| a > b)
}
pub fn less_than<'a, T: CanT + PartialOrd>(a: Can<T>, b: Can<T>) -> Goal<'a, T> {
    constrain(a, b, |a, b| a < b)
}

pub trait RelativeComparison<'a, T: CanT + PartialOrd> {
    fn greater_than(self, other: Can<T>) -> Goal<'a, T>;
    fn less_than(self, other: Can<T>) -> Goal<'a, T>;
}
impl<'a, T: CanT + PartialOrd> RelativeComparison<'a, T> for Can<T> {
    fn greater_than(self, other: Can<T>) -> Goal<'a, T> {
        greater_than(self, other)
    }
    fn less_than(self, other: Can<T>) -> Goal<'a, T> {
        less_than(self, other)
    }
}
impl<'a, T: CanT + PartialOrd> RelativeComparison<'a, T> for LVar {
    fn greater_than(self, other: Can<T>) -> Goal<'a, T> {
        greater_than(self.can(), other)
    }
    fn less_than(self, other: Can<T>) -> Goal<'a, T> {
        less_than(self.can(), other)
    }
}

#[cfg(test)]
mod tests {
    use super::{greater_than, RelativeComparison};
    use crate::{all, Can, CanT, Equals, Goal, LVar, State};
    use itertools::Itertools;

    fn val<T: CanT>(value: T) -> Can<T> {
        Can::Val(value)
    }

    fn resolve<'a, T: CanT + 'a>(goal: Goal<'a, T>, vars: Vec<LVar>) -> Vec<Vec<Can<T>>> {
        let vars = &vars;
        goal.run(State::new())
            .map(|s| {
                vars.into_iter()
                    .map(|v| s.resolve_var(*v).unwrap())
                    .collect()
            })
            .collect()
    }

    #[test]
    fn relative_gt() {
        let x = LVar::labeled("x");

        struct Case<'a, T: CanT> {
            expected: Vec<Vec<Can<T>>>,
            goals: Vec<Goal<'a, T>>,
        };

        let test_cases: Vec<Case<_>> = vec![
            Case {
                expected: vec![vec![val(2)]],
                goals: vec![greater_than(x.can(), val(1)), x.equals(val(2))],
            },
            Case {
                expected: vec![vec![val(2)]],
                goals: vec![x.greater_than(val(1)), x.equals(val(2))],
            },
            Case {
                expected: vec![vec![val(1)]],
                goals: vec![greater_than(val(2), x.can()), x.equals(val(1))],
            },
            Case {
                expected: vec![vec![val(2)]],
                goals: vec![x.equals(val(2)), val(1).less_than(x.can())],
            },
            Case {
                expected: vec![],
                goals: vec![x.equals(val(1)), x.greater_than(val(2))],
            },
        ];

        for Case { goals, expected } in test_cases {
            let goals_len = goals.len();
            for permutation in goals.into_iter().permutations(goals_len) {
                // debug!("{:?}", &permutation);
                dbg!(&permutation);
                assert_eq!(dbg!(resolve(all(permutation), vec![x])), expected);
            }
        }
    }
}
