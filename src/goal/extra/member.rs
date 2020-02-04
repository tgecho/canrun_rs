use crate::can::hoc::HoC;
use crate::{both, equal, var, Can, CanT, Equals, Goal, LVar, State, StateIter};
use std::iter::empty;

pub fn member<T: CanT>(needle: Can<T>, haystack: Can<T>) -> Goal<T> {
    equal(contains(needle), haystack)
}

fn contains<T: CanT + 'static>(needle: Can<T>) -> Can<T> {
    Can::HoC(HoC {
        composed: false,
        var: var(),
        value: Box::new(needle),
        unify: unify_contains,
    })
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
    use crate::{all, any, both, equal, member, pair, var, Can, CanT, Equals, Goal, LVar, State};

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

        let goal: Goal<&str> = all(vec![
            x.equals(pair(Can::Val("name"), z.can())),
            y.equals(Can::Vec(vec![john.clone(), mary.clone(), monkey.clone()])),
            member(x.can(), c.can()),
            member(c.can(), y.can()),
        ]);

        let resolve = |goal: &Goal<_>| -> Vec<_> {
            goal.run(&State::new())
                .map(|s| (s.resolve_var(z).unwrap(), s.resolve_var(c).unwrap()))
                .collect()
        };

        // assert_eq!(
        //     resolve(&goal),
        //     vec![
        //         (Can::Val("john"), john.clone()),
        //         (Can::Val("mary"), mary.clone()),
        //         (Can::Val("monkey"), monkey.clone())
        //     ]
        // );

        let goal = both(
            goal,
            member(pair(Can::Val("is"), Can::Val("hungry")), c.can()),
        );
        let res = resolve(&goal);
        dbg!(&res);
        // TODO: This works if the extra member and goal are swapped :/
        // Also, swapping the two members in the original goals fixes it too :/
        assert_eq!(res, vec![(Can::Val("monkey"), monkey)]);
    }

    fn val<T: CanT>(value: T) -> Can<T> {
        Can::Val(value)
    }

    #[test]
    fn unify_two_contains() {
        // let (x, y, z) = (var(), var(), var());
        let z = LVar::labeled("z");
        let list = Can::Vec(vec![val(1), val(2), val(3)]);

        let goal = all(vec![
            member(val(1), z.can()),
            member(val(1), z.can()),
            z.equals(list.clone()),
        ]);

        let resolve = |goal: &Goal<_>| -> Vec<_> {
            goal.run(&State::new())
                .map(|s| s.resolve_var(z).unwrap())
                .collect()
        };

        assert_eq!(resolve(&goal), vec![list]);
    }
}
