use crate::can::hoc::hoc_fn;
use crate::{equal, Can, CanT, Goal};

fn greater_than<T: CanT + PartialOrd + 'static>(value: Can<T>) -> Can<T> {
    hoc_fn(value, |output, value, other| match (other, value) {
        (Can::Val(o), Can::Val(v)) if o > v => equal(output.can(), Can::Val(o)),
        _ => Goal::Fail,
    })
}

#[cfg(test)]
mod tests {
    use super::greater_than;
    use crate::{all, Can, CanT, Equals, Goal, LVar, State};
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
            Case {
                expected: vec![vec![val(2)]],
                goals: vec![x.equals(val(2)), x.equals(greater_than(val(1)))],
            },
            Case {
                expected: vec![],
                goals: vec![x.equals(val(1)), x.equals(greater_than(val(2)))],
            },
        ];

        for Case { goals, expected } in test_cases {
            let goals_len = goals.len();
            for permutation in goals.into_iter().permutations(goals_len) {
                debug!("{:?}", &permutation);
                dbg!(&permutation);
                assert_eq!(resolve(all(permutation), vec![x]), expected);
            }
        }
    }
}
