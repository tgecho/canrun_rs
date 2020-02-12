use crate::can::hoc::condition_fn;
use crate::{equal, Can, CanT, Goal, LVar};

pub fn greater_than<T: CanT + PartialOrd>(value: Can<T>) -> Can<T> {
    condition_fn(value, |value, other| other > value)
}
pub fn less_than<T: CanT + PartialOrd>(value: Can<T>) -> Can<T> {
    condition_fn(value, |value, other| other < value)
}

pub trait RelativeComparison<T: CanT + PartialOrd> {
    fn greater_than(self, other: Can<T>) -> Goal<T>;
    fn less_than(self, other: Can<T>) -> Goal<T>;
}
impl<T: CanT + PartialOrd> RelativeComparison<T> for Can<T> {
    fn greater_than(self, other: Can<T>) -> Goal<T> {
        equal(self, greater_than(other))
    }
    fn less_than(self, other: Can<T>) -> Goal<T> {
        equal(self, less_than(other))
    }
}
impl<T: CanT + PartialOrd> RelativeComparison<T> for LVar {
    fn greater_than(self, other: Can<T>) -> Goal<T> {
        equal(self.can(), greater_than(other))
    }
    fn less_than(self, other: Can<T>) -> Goal<T> {
        equal(self.can(), less_than(other))
    }
}

#[cfg(test)]
mod tests {
    use super::{greater_than, RelativeComparison};
    use crate::{all, equal, Can, CanT, Equals, Goal, LVar, State};
    use itertools::Itertools;

    fn val<T: CanT>(value: T) -> Can<T> {
        Can::Val(value)
    }

    fn resolve<T: CanT>(goal: Goal<T>, vars: Vec<LVar>) -> Vec<Vec<Can<T>>> {
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

        struct Case<T: CanT + 'static> {
            expected: Vec<Vec<Can<T>>>,
            goals: Vec<Goal<T>>,
        };

        let test_cases: Vec<Case<_>> = vec![
            // Case {
            //     expected: vec![vec![val(2)]],
            //     goals: vec![x.equals(greater_than(val(1))), x.equals(val(2))],
            // },
            // Case {
            //     expected: vec![vec![val(2)]],
            //     goals: vec![x.greater_than(val(1)), x.equals(val(2))],
            // },
            Case {
                expected: vec![vec![val(1)]],
                goals: vec![equal(val(2), greater_than(x.can())), x.equals(val(1))],
            },
            // Case {
            //     expected: vec![vec![val(2)]],
            //     goals: vec![x.equals(val(2)), val(1).less_than(x.can())],
            // },
            // Case {
            //     expected: vec![],
            //     goals: vec![x.equals(val(1)), x.greater_than(val(2))],
            // },
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
