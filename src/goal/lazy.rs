use crate::{CanT, Goal, LVar};
use std::rc::Rc;

pub fn lazy<T, F>(func: F) -> Goal<T>
where
    T: CanT,
    F: Fn() -> Goal<T> + 'static,
{
    Goal::Lazy(Rc::new(func))
}

pub fn with1<T, F>(func: F) -> Goal<T>
where
    T: CanT,
    F: Fn(LVar) -> Goal<T> + 'static,
{
    Goal::Lazy(Rc::new(move || func(LVar::new())))
}

pub fn with2<T, F>(func: F) -> Goal<T>
where
    T: CanT,
    F: Fn(LVar, LVar) -> Goal<T> + 'static,
{
    Goal::Lazy(Rc::new(move || func(LVar::new(), LVar::new())))
}

pub fn with3<T, F>(func: F) -> Goal<T>
where
    T: CanT,
    F: Fn(LVar, LVar, LVar) -> Goal<T> + 'static,
{
    Goal::Lazy(Rc::new(move || func(LVar::new(), LVar::new(), LVar::new())))
}

#[cfg(test)]
mod tests {
    use crate::{both, equal, lazy, Can, LVar, State};

    #[test]
    fn basic_lazy() {
        let y = LVar::new();
        let goal = lazy(move || {
            let x = Can::Var(LVar::new());
            let yy = Can::Var(y);
            both(equal(x.clone(), Can::Val(5)), equal(x, yy))
        });

        let mut result1 = goal.run(&State::new());
        assert_eq!(result1.nth(0).unwrap().resolve_var(y), Can::Val(5));

        // This shows that we can run the same lazy goal again
        let mut result2 = goal.run(&State::new());
        assert_eq!(result2.nth(0).unwrap().resolve_var(y), Can::Val(5));
    }
}
