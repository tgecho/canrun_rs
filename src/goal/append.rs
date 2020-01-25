use super::{both, either, equal, with3, Goal};
use crate::cell::pair::Pair;
use crate::Cell;

pub fn append<T: Eq + Clone>(a: Cell<T>, b: Cell<T>, c: Cell<T>) -> Goal<T> {
    either(
        both(equal(a.clone(), Cell::Nil), equal(b.clone(), c.clone())),
        with3(move |first, rest_of_a, rest_of_c| {
            both(
                both(
                    equal(a.clone(), Pair::new(first.into(), rest_of_a.into())),
                    equal(c.clone(), Pair::new(first.into(), rest_of_c.into())),
                ),
                append(rest_of_a.into(), b.clone(), rest_of_c.into()),
            )
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
            Pair::new(Cell::Value(Some("i")), Cell::Nil),
        );
        let h = Pair::new(Cell::Value(Some("h")), Cell::Nil);
        let i = Pair::new(Cell::Value(Some("i")), Cell::Nil);
        let goal = append(h, x.into(), hi);

        let mut result1 = goal.clone().run(&state);
        assert_eq!(result1.nth(0).unwrap().resolve_var(x), i);
    }
}
