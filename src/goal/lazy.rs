use crate::{var, CanT, Goal, LVar};
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
    Goal::Lazy(Rc::new(move || func(var())))
}

pub fn with2<T, F>(func: F) -> Goal<T>
where
    T: CanT,
    F: Fn(LVar, LVar) -> Goal<T> + 'static,
{
    Goal::Lazy(Rc::new(move || func(var(), var())))
}

pub fn with3<'a, T, F>(func: F) -> Goal<T>
where
    T: CanT + 'static,
    F: Fn(LVar, LVar, LVar) -> Goal<T> + 'static,
{
    Goal::Lazy(Rc::new(move || func(var(), var(), var())))
}

#[cfg(test)]
mod tests {
    use crate::{both, lazy, var, Can, Equals, State};

    #[test]
    fn basic_lazy() {
        let y = var();
        let goal = lazy(move || {
            let x = var();
            both(x.equals(5), x.equals(y.can()))
        });

        let mut result1 = goal.clone().run(State::new());
        assert_eq!(result1.nth(0).unwrap().resolve_var(y).unwrap(), Can::Val(5));

        // This shows that we can run the same lazy goal again
        let mut result2 = goal.run(State::new());
        assert_eq!(result2.nth(0).unwrap().resolve_var(y).unwrap(), Can::Val(5));
    }
}
