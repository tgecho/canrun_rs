use super::Goal;
use crate::Can;
use std::rc::Rc;

pub fn member<T: Eq + Clone, I>(needle: Can<T>, haystack: I) -> Goal<T>
where
    I: 'static + Clone + IntoIterator<Item = Can<T>>,
{
    Goal::Member {
        needle,
        iter: Rc::new(move || Box::new(haystack.clone().into_iter())),
    }
}

#[cfg(test)]
mod tests {
    use super::member;
    use crate::{any, both, equal, Can, LVar, Pair, State};
    #[test]
    fn basic_member() {
        let x = LVar::new();
        let goal = member(Can::Var(x), vec![Can::Val(1), Can::Val(2), Can::Val(3)]);
        let result: Vec<_> = goal.run(State::new()).map(|r| r.resolve_var(x)).collect();
        assert_eq!(result, vec![Can::Val(1), Can::Val(2), Can::Val(3)]);
    }
    #[test]
    fn member_with_conditions() {
        let x = LVar::new();
        let goal = both(
            equal(Can::Var(x), Can::Val(2)),
            member(Can::Var(x), vec![Can::Val(1), Can::Val(2), Can::Val(3)]),
        );
        let result: Vec<_> = goal.run(State::new()).map(|r| r.resolve_var(x)).collect();
        assert_eq!(result, vec![Can::Val(2)]);
    }
    #[test]
    fn member_with_pairs() {
        let x = LVar::new();
        let y = LVar::new();

        fn rel<T: Eq + Clone>(a: Can<T>, b: Can<T>, c: Can<T>) -> Can<T> {
            Pair::new(a, Pair::new(b, c))
        }

        let find = |desired| {
            let goal = both(
                equal(x.into(), desired),
                member(
                    x.into(),
                    vec![
                        rel(Can::Val(0), Can::Val(1), Can::Val(2)),
                        rel(Can::Val(3), Can::Val(4), Can::Val(5)),
                    ],
                ),
            );
            let result: Vec<_> = goal
                .run(State::new())
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

        fn rel<T: Eq + Clone>(a: Can<T>, b: Can<T>, c: Can<T>) -> Can<T> {
            Pair::new(a, Pair::new(b, c))
        }

        let goal = both(
            any(vec![
                equal(x.into(), rel(Can::Val(0), y.into(), Can::Val(2))),
                equal(x.into(), rel(y.into(), Can::Val(1), Can::Val(2))),
                equal(x.into(), rel(Can::Val(3), Can::Val(4), y.into())),
            ]),
            member(
                x.into(),
                vec![
                    rel(Can::Val(0), Can::Val(1), Can::Val(2)),
                    rel(Can::Val(3), Can::Val(4), Can::Val(5)),
                ],
            ),
        );
        let result: Vec<_> = goal.run(State::new()).map(|r| r.resolve_var(y)).collect();

        assert_eq!(result, vec![Can::Val(5), Can::Val(0), Can::Val(1)]);
    }
}
