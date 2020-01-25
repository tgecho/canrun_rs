use super::{both, either, equal, lazy, Goal};
use crate::cell::pair::Pair;
use crate::{Cell, LVar};

pub fn append<T: Eq + Clone>(
    a: Cell<Option<T>>,
    b: Cell<Option<T>>,
    c: Cell<Option<T>>,
) -> Goal<Option<T>> {
    let empty: Cell<Option<T>> = Cell::Value(None);
    either(
        both(equal(a.clone(), empty), equal(b.clone(), c.clone())),
        lazy(move || {
            let first = LVar::new();
            let rest_of_a = LVar::new();
            let rest_of_c = LVar::new();
            return both(
                both(
                    equal(a.clone(), Pair::new(first.into(), rest_of_a.into())),
                    equal(c.clone(), Pair::new(first.into(), rest_of_c.into())),
                ),
                append(rest_of_a.into(), b.clone(), rest_of_c.into()),
            );
        }),
    )
}

#[cfg(test)]
mod tests {
    use super::append;
    use crate::cell::pair::Pair;
    use crate::{Cell, LVar, State};

    #[test]
    fn basic_append() {
        let state: State<Option<&str>> = State::new();
        let x = LVar::new();
        let hi = Pair::new(
            Cell::Value(Some("h")),
            Pair::new(Cell::Value(Some("i")), Cell::Value(None)),
        );
        let h = Pair::new(Cell::Value(Some("h")), Cell::Value(None));
        let i = Pair::new(Cell::Value(Some("i")), Cell::Value(None));
        let goal = append(h, x.into(), hi);

        let mut result1 = goal.clone().run(&state);
        assert_eq!(result1.nth(0).unwrap().resolve_var(x), i);
    }
}
