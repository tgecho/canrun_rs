use super::Goal;
use crate::LVar;
use std::rc::Rc;

pub fn lazy<T, F>(func: F) -> Goal<T>
where
    T: Eq + Clone,
    F: Fn() -> Goal<T> + 'static,
{
    Goal::Lazy(Rc::new(func))
}

pub fn with1<T, F>(func: F) -> Goal<T>
where
    T: Eq + Clone,
    F: Fn(LVar) -> Goal<T> + 'static,
{
    Goal::Lazy(Rc::new(move || func(LVar::new())))
}

pub fn with2<T, F>(func: F) -> Goal<T>
where
    T: Eq + Clone,
    F: Fn(LVar, LVar) -> Goal<T> + 'static,
{
    Goal::Lazy(Rc::new(move || func(LVar::new(), LVar::new())))
}

pub fn with3<T, F>(func: F) -> Goal<T>
where
    T: Eq + Clone,
    F: Fn(LVar, LVar, LVar) -> Goal<T> + 'static,
{
    Goal::Lazy(Rc::new(move || func(LVar::new(), LVar::new(), LVar::new())))
}

#[cfg(test)]
mod tests {
    use crate::{both, equal, lazy, Cell, LVar, State};

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
