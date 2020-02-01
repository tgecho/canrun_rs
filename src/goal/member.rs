use crate::{equal, Can, CanT, Goal, GoalIter, State};
use std::iter::empty;

pub fn member<T: CanT>(needle: Can<T>, haystack: Can<T>) -> Goal<T> {
    equal(contains(needle), haystack)
}

fn unify_contains<T: CanT + 'static>(
    needle: Can<T>,
    other: Can<T>,
    state: State<T>,
) -> GoalIter<T> {
    match other {
        Can::Vec(haystack) => Box::new(
            haystack
                .into_iter()
                .flat_map(move |c| state.unify(&needle, &c)),
        ),
        _ => Box::new(empty()),
    }
}

fn contains<T: CanT + 'static>(needle: Can<T>) -> Can<T> {
    Can::HoC {
        value: Box::new(needle),
        unify: unify_contains,
    }
}

pub fn membero<T: CanT>(needle: Can<T>, vec: Can<T>) -> Goal<T> {
    equal(contains(needle), vec)
}

#[cfg(test)]
mod tests {
    use super::member;
    use crate::goal::append::pair;
    use crate::{all, any, both, equal, Can, CanT, LVar, State};
    #[test]
    fn basic_member() {
        let x = LVar::new();
        let goal = member(
            Can::Var(x),
            Can::Vec(vec![Can::Val(1), Can::Val(2), Can::Val(3)]),
        );
        let result: Vec<_> = goal.run(&State::new()).map(|r| r.resolve_var(x)).collect();
        assert_eq!(result, vec![Can::Val(1), Can::Val(2), Can::Val(3)]);
    }
    #[test]
    fn member_with_conditions() {
        let x = LVar::new();
        let goal = both(
            equal(Can::Var(x), Can::Val(2)),
            member(
                Can::Var(x),
                Can::Vec(vec![Can::Val(1), Can::Val(2), Can::Val(3)]),
            ),
        );
        let result: Vec<_> = goal.run(&State::new()).map(|r| r.resolve_var(x)).collect();
        assert_eq!(result, vec![Can::Val(2)]);
    }
    #[test]
    fn member_with_pairs() {
        let x = LVar::new();
        let y = LVar::new();

        fn rel<T: CanT>(a: Can<T>, b: Can<T>, c: Can<T>) -> Can<T> {
            pair(a, pair(b, c))
        }

        let find = |desired| {
            let goal = both(
                equal(x.into(), desired),
                member(
                    x.into(),
                    Can::Vec(vec![
                        rel(Can::Val(0), Can::Val(1), Can::Val(2)),
                        rel(Can::Val(3), Can::Val(4), Can::Val(5)),
                    ]),
                ),
            );
            let result: Vec<_> = goal
                .run(&State::new())
                .map(|r| (r.resolve_var(y), r.resolve_var(x)))
                .collect();
            result
        };

        assert_eq!(
            find(rel(y.into(), Can::Val(1), Can::Val(2))),
            vec![(Can::Val(0), rel(Can::Val(0), Can::Val(1), Can::Val(2)))]
        );

        assert_eq!(
            find(rel(Can::Val(0), y.into(), Can::Val(2))),
            vec![(Can::Val(1), rel(Can::Val(0), Can::Val(1), Can::Val(2)))]
        );

        assert_eq!(find(rel(Can::Val(1), y.into(), Can::Val(2))), vec![]);
    }
    #[test]
    fn member_with_pairs_complex() {
        let x = LVar::new();
        let y = LVar::new();

        fn rel<T: CanT>(a: Can<T>, b: Can<T>, c: Can<T>) -> Can<T> {
            pair(a, pair(b, c))
        }

        let goal = both(
            any(vec![
                equal(x.into(), rel(Can::Val(0), y.into(), Can::Val(2))),
                equal(x.into(), rel(y.into(), Can::Val(1), Can::Val(2))),
                equal(x.into(), rel(Can::Val(3), Can::Val(4), y.into())),
            ]),
            member(
                x.into(),
                Can::Vec(vec![
                    rel(Can::Val(0), Can::Val(1), Can::Val(2)),
                    rel(Can::Val(3), Can::Val(4), Can::Val(5)),
                ]),
            ),
        );
        let result: Vec<_> = goal.run(&State::new()).map(|r| r.resolve_var(y)).collect();

        assert_eq!(result, vec![Can::Val(5), Can::Val(0), Can::Val(1)]);
    }

    use super::{contains, membero};
    #[test]
    fn member_with_pairs_as_map() {
        let x = LVar::labeled("x");
        let y = LVar::labeled("y");
        let z = LVar::labeled("z");

        let john = Can::Vec(vec![
            pair(Can::Val("name"), Can::Val("john")),
            pair(Can::Val("wat"), Can::Val("foo")),
            pair(Can::Val("is"), Can::Val("hungry")),
        ]);

        let mary = Can::Vec(vec![
            pair(Can::Val("name"), Can::Val("mary")),
            pair(Can::Val("wat"), Can::Val("the")),
        ]);

        let monkey = Can::Vec(vec![
            pair(Can::Val("name"), Can::Val("monkey")),
            pair(Can::Val("is"), Can::Val("super")),
        ]);

        // let goal = both(
        //     member(x.into(), vec![john]),
        //     membero(pair(Can::Val("name"), Can::Val("john")), x.into()),
        //     // equal(y.into(), pair(Can::Val("is"), Can::Val("hungry"))),
        // );

        // let goal = all(vec![
        //     equal(x.into(), pair(Can::Val("name"), z.into())),
        //     equal(y.into(), john),
        //     membero(x.into(), y.into()),
        // ]);

        let goal = all(vec![
            equal(x.into(), pair(Can::Val("name"), z.into())),
            equal(y.into(), Can::Vec(vec![john, mary, monkey])),
            membero(contains(x.into()), y.into()),
        ]);

        // let goal = membero(
        //     Can::Val("name"),
        //     Can::Vec(vec![Can::Val("name"), Can::Val("john")]),
        // );

        // let goal = both(
        //     both(
        //         equal(x.into(), Can::Val("name")),
        //         membero(x.into(), y.into()),
        //     ),
        //     equal(y.into(), Can::Vec(vec![Can::Val("name"), Can::Val("john")])),
        // );
        let result: Vec<_> = goal.run(&State::new()).collect();

        dbg!(goal
            .run(&State::new())
            .map(|s| s.resolve_var(z))
            .collect::<Vec<_>>());

        assert_ne!(result, vec![]);
    }
}
