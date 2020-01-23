use super::{both, either, equal, lazy, Goal};
use crate::lvar::LVar;
use crate::state::{pair, Cell};

pub fn append<T: Eq + Clone>(
    a: Cell<Option<T>>,
    b: Cell<Option<T>>,
    c: Cell<Option<T>>,
) -> Goal<Option<T>> {
    let empty: Cell<Option<T>> = Cell::Value(None);
    either(
        both(equal(a.clone(), empty), equal(b.clone(), c.clone())),
        lazy(move || {
            let first = Cell::Var(LVar::new());
            let rest_of_a = Cell::Var(LVar::new());
            let rest_of_c = Cell::Var(LVar::new());
            return both(
                both(
                    equal(a.clone(), pair(first.clone(), rest_of_a.clone())),
                    equal(c.clone(), pair(first, rest_of_c.clone())),
                ),
                append(rest_of_a, b.clone(), rest_of_c),
            );
        }),
    )
}

#[cfg(test)]
mod tests {
    use super::append;
    use crate::lvar::LVar;
    use crate::state::{pair, Cell, State};

    #[test]
    fn basic_append() {
        let state: State<Option<&str>> = State::new();
        let x = Cell::Var(LVar::new());
        let hi = pair(
            Cell::Value(Some("h")),
            pair(Cell::Value(Some("i")), Cell::Value(None)),
        );
        let h = pair(Cell::Value(Some("h")), Cell::Value(None));
        let i = pair(Cell::Value(Some("i")), Cell::Value(None));
        let goal = append(h, x.clone(), hi);

        let mut result1 = goal.clone().run(&state);
        assert_eq!(result1.nth(0).unwrap().resolve(&x), i);
    }
}
