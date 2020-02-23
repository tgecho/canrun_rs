use crate::{CanT, Goal, State, StateIter};
use std::rc::Rc;

pub fn custom<'a, T, F>(func: F) -> Goal<'a, T>
where
    T: CanT + 'a,
    F: Fn(State<'a, T>) -> StateIter<'a, T> + 'a,
{
    Goal::Custom(Rc::new(func))
}

#[cfg(test)]
mod tests {
    use super::custom;
    use crate::{both, var, Can, Equals, LVar, State};

    #[test]
    fn basic_custom() {
        let y = LVar::new();
        let goal = custom(|state| {
            let x = var();
            both(x.equals(5), x.equals(y.can())).run(state)
        });

        let mut result1 = goal.clone().run(State::new());
        assert_eq!(result1.nth(0).unwrap().resolve_var(y).unwrap(), Can::Val(5));

        // This shows that we can run the same custom goal again
        let mut result2 = goal.run(State::new());
        assert_eq!(result2.nth(0).unwrap().resolve_var(y).unwrap(), Can::Val(5));
    }
}
