use crate::can::pair::pair;
use crate::{both, either, equal, with3, Goal};
use crate::{Can, CanT};

pub fn append<'a, T: CanT + 'a>(a: Can<T>, b: Can<T>, c: Can<T>) -> Goal<'a, T> {
    either(
        both(equal(a.clone(), Can::Nil), equal(b.clone(), c.clone())),
        with3(move |first, rest_of_a, rest_of_c| {
            both(
                both(
                    equal(a.clone(), pair(first.can(), rest_of_a.can())),
                    equal(c.clone(), pair(first.can(), rest_of_c.can())),
                ),
                append(rest_of_a.can(), b.clone(), rest_of_c.can()),
            )
        }),
    )
}

#[cfg(test)]
mod tests {
    use super::{append, pair};
    use crate::{var, Can, State};

    #[test]
    fn basic_append() {
        let state: State<Option<&str>> = State::new();
        let x = var();
        let hi = pair(Can::Val(Some("h")), pair(Can::Val(Some("i")), Can::Nil));
        let h = pair(Can::Val(Some("h")), Can::Nil);
        let i = pair(Can::Val(Some("i")), Can::Nil);
        let goal = append(h, x.can(), hi);

        let mut result1 = goal.run(state);
        assert_eq!(result1.nth(0).unwrap().resolve_var(x).unwrap(), i);
    }
}
