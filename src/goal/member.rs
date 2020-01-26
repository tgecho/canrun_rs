use super::Goal;
use crate::Cell;
use std::rc::Rc;

pub fn member<T: Eq + Clone, I>(needle: Cell<T>, haystack: I) -> Goal<T>
where
    I: 'static + Clone + Iterator<Item = Cell<T>>,
{
    Goal::Member {
        needle,
        iter: Rc::new(move || Box::new(haystack.clone())),
    }
}

#[cfg(test)]
mod tests {
    use super::member;
    use crate::{any, both, equal, Cell, LVar, Pair, State};
    #[test]
    fn basic_member() {
        let x = LVar::new();
        let goal = member(
            Cell::Var(x),
            vec![Cell::Value(1), Cell::Value(2), Cell::Value(3)].into_iter(),
        );
        let result: Vec<_> = goal.run(State::new()).map(|r| r.resolve_var(x)).collect();
        assert_eq!(result, vec![Cell::Value(1), Cell::Value(2), Cell::Value(3)]);
    }
    #[test]
    fn member_with_conditions() {
        let x = LVar::new();
        let goal = both(
            equal(Cell::Var(x), Cell::Value(2)),
            member(
                Cell::Var(x),
                vec![Cell::Value(1), Cell::Value(2), Cell::Value(3)].into_iter(),
            ),
        );
        let result: Vec<_> = goal.run(State::new()).map(|r| r.resolve_var(x)).collect();
        assert_eq!(result, vec![Cell::Value(2)]);
    }
    #[test]
    fn member_with_pairs() {
        let x = LVar::new();
        let y = LVar::new();

        fn rel<T: Eq + Clone>(a: Cell<T>, b: Cell<T>, c: Cell<T>) -> Cell<T> {
            Pair::new(a, Pair::new(b, c))
        }

        let find = |desired| {
            let goal = both(
                equal(x.into(), desired),
                member(
                    x.into(),
                    vec![
                        rel(Cell::Value(0), Cell::Value(1), Cell::Value(2)),
                        rel(Cell::Value(3), Cell::Value(4), Cell::Value(5)),
                    ]
                    .into_iter(),
                ),
            );
            let result: Vec<_> = goal
                .run(State::new())
                .map(|r| (r.resolve_var(y), r.resolve_var(x)))
                .collect();
            result
        };

        assert_eq!(
            find(rel(y.into(), Cell::Value(1), Cell::Value(2))),
            vec![(
                Cell::Value(0),
                rel(Cell::Value(0), Cell::Value(1), Cell::Value(2))
            )]
        );

        assert_eq!(
            find(rel(Cell::Value(0), y.into(), Cell::Value(2))),
            vec![(
                Cell::Value(1),
                rel(Cell::Value(0), Cell::Value(1), Cell::Value(2))
            )]
        );

        assert_eq!(find(rel(Cell::Value(1), y.into(), Cell::Value(2))), vec![]);
    }
    #[test]
    fn member_with_pairs_complex() {
        let x = LVar::new();
        let y = LVar::new();

        fn rel<T: Eq + Clone>(a: Cell<T>, b: Cell<T>, c: Cell<T>) -> Cell<T> {
            Pair::new(a, Pair::new(b, c))
        }

        let goal = both(
            any(vec![
                equal(x.into(), rel(Cell::Value(0), y.into(), Cell::Value(2))),
                equal(x.into(), rel(y.into(), Cell::Value(1), Cell::Value(2))),
                equal(x.into(), rel(Cell::Value(3), Cell::Value(4), y.into())),
            ]),
            member(
                x.into(),
                vec![
                    rel(Cell::Value(0), Cell::Value(1), Cell::Value(2)),
                    rel(Cell::Value(3), Cell::Value(4), Cell::Value(5)),
                ]
                .into_iter(),
            ),
        );
        let result: Vec<_> = goal.run(State::new()).map(|r| r.resolve_var(y)).collect();

        assert_eq!(result, vec![Cell::Value(5), Cell::Value(0), Cell::Value(1)]);
    }
}
