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
    use crate::lvar::LVar;
    use crate::state::{Cell, State};

    #[test]
    fn basic_lazy() {
        let state: State<u32> = State::new();
        let y = LVar::new();
        let goal = lazy(move || {
            let x = Cell::Var(LVar::new());
            let yy = Cell::Var(y);
            // let yy = Cell::Var(LVar::new());
            both(equal(x.clone(), Cell::Value(5)), equal(x, yy))
        });

        let mut result1 = goal.clone().run(&state);
        assert_eq!(result1.nth(0).unwrap().resolve_var(y), Cell::Value(5));

        // This shows that lazy a clone we can run the same goal again
        let mut result2 = goal.run(&state);
        assert_eq!(result2.nth(0).unwrap().resolve_var(y), Cell::Value(5));
    }
}
