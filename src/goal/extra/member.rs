use crate::can::hoc::hoc_fn;
use crate::{both, equal, Can, CanT, Goal, LVar, State, StateIter};
use std::iter::empty;

pub fn member<T: CanT>(needle: Can<T>, haystack: Can<T>) -> Goal<T> {
    equal(contains(needle), haystack)
}

fn contains<T: CanT + 'static>(needle: Can<T>) -> Can<T> {
    hoc_fn(needle, unify_contains)
}

fn unify_contains<T: CanT + 'static>(
    var: LVar,
    needle: Can<T>,
    other: Can<T>,
    state: State<T>,
) -> StateIter<T> {
    match other.clone() {
        Can::Vec(haystack) => {
            let iter = (haystack.into_iter()).flat_map(move |can| {
                both(equal(needle.clone(), can), equal(other.clone(), var.can())).run(&state)
            });
            Box::new(iter)
        }
        _ => Box::new(empty()),
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        all, any, both, either, equal, member, pair, var, Can, CanT, Equals, Goal, LVar, State,
    };
    use itertools::Itertools;

    #[test]
    fn basic_member() {
        let x = var();
        let goal = member(
            Can::Var(x),
            Can::Vec(vec![Can::Val(1), Can::Val(2), Can::Val(3)]),
        );
        let result: Vec<_> = goal
            .run(&State::new())
            .map(|r| r.resolve_var(x).unwrap())
            .collect();
        assert_eq!(result, vec![Can::Val(1), Can::Val(2), Can::Val(3)]);
    }
    #[test]
    fn member_with_conditions() {
        let x = var();
        let goal = both(
            equal(Can::Var(x), Can::Val(2)),
            member(
                Can::Var(x),
                Can::Vec(vec![Can::Val(1), Can::Val(2), Can::Val(3)]),
            ),
        );
        let result: Vec<_> = goal
            .run(&State::new())
            .map(|r| r.resolve_var(x).unwrap())
            .collect();
        assert_eq!(result, vec![Can::Val(2)]);
    }
    #[test]
    fn member_with_pairs() {
        let x = var();
        let y = var();

        fn rel<T: CanT>(a: Can<T>, b: Can<T>, c: Can<T>) -> Can<T> {
            pair(a, pair(b, c))
        }

        let find = |desired: Can<_>| {
            let goal = both(
                x.equals(desired),
                member(
                    x.can(),
                    Can::Vec(vec![
                        rel(Can::Val(0), Can::Val(1), Can::Val(2)),
                        rel(Can::Val(3), Can::Val(4), Can::Val(5)),
                    ]),
                ),
            );
            let result: Vec<_> = goal
                .run(&State::new())
                .map(|r| (r.resolve_var(y).unwrap(), r.resolve_var(x).unwrap()))
                .collect();
            result
        };

        assert_eq!(
            find(rel(y.can(), Can::Val(1), Can::Val(2))),
            vec![(Can::Val(0), rel(Can::Val(0), Can::Val(1), Can::Val(2)))]
        );

        assert_eq!(
            find(rel(Can::Val(0), y.can(), Can::Val(2))),
            vec![(Can::Val(1), rel(Can::Val(0), Can::Val(1), Can::Val(2)))]
        );

        assert_eq!(find(rel(Can::Val(1), y.can(), Can::Val(2))), vec![]);
    }
    #[test]
    fn member_with_pairs_complex() {
        let x = var();
        let y = var();

        fn rel<T: CanT>(a: Can<T>, b: Can<T>, c: Can<T>) -> Can<T> {
            pair(a, pair(b, c))
        }

        let goal: Goal<usize> = both(
            any(vec![
                x.equals(rel(Can::Val(0), y.can(), Can::Val(2))),
                x.equals(rel(y.can(), Can::Val(1), Can::Val(2))),
                x.equals(rel(Can::Val(3), Can::Val(4), y.can())),
            ]),
            member(
                x.can(),
                Can::Vec(vec![
                    rel(Can::Val(0), Can::Val(1), Can::Val(2)),
                    rel(Can::Val(3), Can::Val(4), Can::Val(5)),
                ]),
            ),
        );
        let result: Vec<_> = goal
            .run(&State::new())
            .map(|r| r.resolve_var(y).unwrap())
            .collect();

        assert_eq!(result, vec![Can::Val(5), Can::Val(0), Can::Val(1)]);
    }

    fn get_records() -> (Can<&'static str>, Can<&'static str>, Can<&'static str>) {
        let john = Can::Vec(vec![
            pair(Can::Val("name"), Can::Val("john")),
            pair(Can::Val("wat"), Can::Val("foo")),
        ]);

        let mary = Can::Vec(vec![
            pair(Can::Val("name"), Can::Val("mary")),
            pair(Can::Val("is"), Can::Val("super")),
        ]);

        let monkey = Can::Vec(vec![
            pair(Can::Val("name"), Can::Val("monkey")),
            pair(Can::Val("is"), Can::Val("hungry")),
        ]);

        (john, mary, monkey)
    }

    #[test]
    fn member_with_additional_contraints() {
        let (x, y) = (var(), var());
        let (john, mary, monkey) = get_records();

        let goal = both(
            member(x.can(), Can::Vec(vec![john, mary, monkey])),
            member(pair(Can::Val("name"), y.can()), x.can()),
        );

        let resolve = |goal: &Goal<&'static str>| -> Vec<Can<&'static str>> {
            goal.run(&State::new())
                .map(|s| s.resolve_var(y).unwrap())
                .collect()
        };

        assert_eq!(
            resolve(&goal),
            vec![Can::Val("john"), Can::Val("mary"), Can::Val("monkey")]
        );

        // We can also add extra conditions (NOTE: should think about efficiency long term)
        let goal = both(
            goal,
            member(pair(Can::Val("is"), Can::Val("hungry")), x.can()),
        );
        assert_eq!(resolve(&goal), vec![Can::Val("monkey")]);
    }

    #[test]
    fn member_with_vars_in_both_positions() {
        let (c, x, y, z) = (
            LVar::labeled("c"),
            LVar::labeled("x"),
            LVar::labeled("y"),
            LVar::labeled("z"),
        );
        let (john, mary, monkey) = get_records();

        struct Case<T: CanT + 'static> {
            goals: Vec<Goal<T>>,
            results: Vec<LVar>,
            expected: Vec<Vec<Can<T>>>,
        };

        let test_cases: Vec<Case<_>> = vec![
            Case {
                goals: vec![
                    x.equals(pair(Can::Val("name"), z.can())),
                    y.equals(Can::Vec(vec![john.clone(), mary.clone(), monkey.clone()])),
                    member(x.can(), c.can()),
                    member(c.can(), y.can()),
                ],
                results: vec![z, c],
                expected: vec![
                    vec![Can::Val("john"), john.clone()],
                    vec![Can::Val("mary"), mary.clone()],
                    vec![Can::Val("monkey"), monkey.clone()],
                ],
            },
            Case {
                goals: vec![
                    x.equals(pair(Can::Val("name"), z.can())),
                    y.equals(Can::Vec(vec![john.clone(), mary.clone(), monkey.clone()])),
                    member(x.can(), c.can()),
                    member(c.can(), y.can()),
                    member(pair(Can::Val("is"), Can::Val("hungry")), c.can()),
                ],
                results: vec![z, c],
                expected: vec![vec![Can::Val("monkey"), monkey.clone()]],
            },
        ];

        for Case {
            goals,
            results,
            expected,
        } in test_cases
        {
            let goals_len = goals.len();
            for permutation in goals.into_iter().permutations(goals_len) {
                dbg!(&permutation);
                assert_eq!(resolve(&all(permutation), results.clone()), expected);
            }
        }
    }

    fn val<T: CanT>(value: T) -> Can<T> {
        Can::Val(value)
    }

    fn resolve<T: CanT>(goal: &Goal<T>, vars: Vec<LVar>) -> Vec<Vec<Can<T>>> {
        let vars = &vars;
        goal.run(&State::new())
            .map(|s| {
                vars.into_iter()
                    .map(|v| s.resolve_var(*v).unwrap())
                    .collect()
            })
            .collect()
    }

    #[test]
    fn unify_two_contains() {
        let z = LVar::labeled("z");
        let list = Can::Vec(vec![val(1), val(2), val(3)]);

        struct Case<T: CanT + 'static> {
            expected: Vec<Vec<Can<T>>>,
            goals: Vec<Goal<T>>,
        };

        let test_cases: Vec<Case<_>> = vec![
            Case {
                expected: vec![vec![list.clone()]],
                goals: vec![
                    member(val(1), z.can()),
                    member(val(1), z.can()),
                    z.equals(list.clone()),
                ],
            },
            Case {
                expected: vec![vec![list.clone()]],
                goals: vec![
                    member(val(1), z.can()),
                    member(val(2), z.can()),
                    z.equals(list.clone()),
                ],
            },
            Case {
                expected: vec![vec![list.clone()]],
                goals: vec![
                    either(member(val(1), z.can()), member(val(4), z.can())),
                    member(val(2), z.can()),
                    z.equals(list.clone()),
                ],
            },
            Case {
                expected: vec![],
                goals: vec![
                    member(val(1), z.can()),
                    member(val(4), z.can()),
                    z.equals(list.clone()),
                ],
            },
        ];

        for Case { goals, expected } in test_cases {
            let goals_len = goals.len();
            for permutation in goals.into_iter().permutations(goals_len) {
                dbg!(&permutation);
                assert_eq!(resolve(&all(permutation), vec![z]), expected);
            }
        }
    }
}
