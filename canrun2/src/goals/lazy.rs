use std::fmt::Debug;
use std::rc::Rc;

use crate::core::State;

use super::Goal;

type LazyFun = dyn Fn() -> Box<dyn Goal>;
pub struct Lazy {
    fun: Rc<LazyFun>,
}

impl Debug for Lazy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Lazy")
            .field("fun", &"Rc<dyn Fn() -> Box<dyn Goal>>")
            .finish()
    }
}

pub fn lazy<F>(fun: F) -> Lazy
where
    F: (Fn() -> Box<dyn Goal>) + 'static,
{
    Lazy { fun: Rc::new(fun) }
}

impl Goal for Lazy {
    fn apply(&self, state: State) -> Option<State> {
        let fun = &self.fun;
        let goal = fun();
        goal.apply(state)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        goals::{succeed::Succeed, unify::unify},
        value::{LVar, Value},
    };

    use super::*;

    #[test]
    fn succeeds() {
        let x = LVar::new();
        let goal = lazy(move || Box::new(unify(x, 1)));
        let result = goal.apply(State::new());
        assert_eq!(result.unwrap().resolve(&x.into()), Value::new(1));
    }

    #[test]
    fn debug_impl() {
        let goal = lazy(|| Box::new(Succeed));
        assert_ne!(format!("{:?}", goal), "")
    }
}
