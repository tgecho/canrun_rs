use super::Goal;
use std::rc::Rc;

pub fn lazy<T, F>(func: F) -> Goal<T>
where
    T: Eq + Clone,
    F: Fn() -> Goal<T> + 'static,
{
    Goal::Lazy(LazyGoal(Rc::new(func)))
}

#[derive(Clone)]
pub struct LazyGoal<T: Eq + Clone + 'static>(pub Rc<dyn Fn() -> Goal<T>>);

#[cfg(test)]
mod tests {
    use super::lazy;
    use crate::goal::{both, equal};
    use crate::{Cell, LVar, State};

    #[test]
    fn basic_lazy() {
        let state: State<u32> = State::new();
        let y = LVar::new();
        let goal = lazy(move || {
            let x = Cell::Var(LVar::new());
            let yy = Cell::Var(y);
            both(equal(x.clone(), Cell::Value(5)), equal(x, yy))
        });

        let mut result1 = goal.clone().run(&state);
        assert_eq!(result1.nth(0).unwrap().resolve_var(y), Cell::Value(5));

        // This shows that we can run the same lazy goal again
        let mut result2 = goal.run(&state);
        assert_eq!(result2.nth(0).unwrap().resolve_var(y), Cell::Value(5));
    }
}
