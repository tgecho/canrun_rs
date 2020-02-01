use crate::{CanT, Goal, State, StateIter};
use std::rc::Rc;

pub fn custom<T, F>(func: F) -> Goal<T>
where
    T: CanT,
    F: Fn(&State<T>) -> StateIter<T> + 'static,
{
    Goal::Custom(Rc::new(func))
}

#[cfg(test)]
mod tests {
    use super::custom;
    use crate::{both, equal, Can, LVar, State};

    #[test]
    fn basic_custom() {
        let y = LVar::new();
        let goal = custom(move |state| {
            let x = Can::Var(LVar::new());
            both(equal(x.clone(), Can::Val(5)), equal(x, y.into())).run(state)
        });

        let mut result1 = goal.run(&State::new());
        assert_eq!(result1.nth(0).unwrap().resolve_var(y).unwrap(), Can::Val(5));

        // This shows that we can run the same custom goal again
        let mut result2 = goal.run(&State::new());
        assert_eq!(result2.nth(0).unwrap().resolve_var(y).unwrap(), Can::Val(5));
    }
}
