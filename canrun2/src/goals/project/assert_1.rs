use crate::core::{resolve_1, Constraint, ResolveFn, State, Unify, VarWatch};
use crate::goals::Goal;
use crate::value::Value;
use std::fmt::{self, Debug};
use std::rc::Rc;

pub struct Assert1<T: Unify> {
    a: Value<T>,
    f: Rc<dyn Fn(&T) -> bool>,
}

impl<T: Unify> Clone for Assert1<T> {
    fn clone(&self) -> Self {
        Self {
            a: self.a.clone(),
            f: self.f.clone(),
        }
    }
}

impl<T: Unify> Assert1<T> {
    pub fn new<F>(a: Value<T>, func: F) -> Self
    where
        F: (Fn(&T) -> bool) + 'static,
    {
        Assert1 {
            a,
            f: Rc::new(func),
        }
    }
}

impl<T: Unify> Debug for Assert1<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Assert1 {:?}", self.a)
    }
}

impl<T: Unify> Constraint for Assert1<T> {
    fn attempt(&self, state: &State) -> Result<ResolveFn, VarWatch> {
        let a = resolve_1(&self.a, state)?;
        let assert = self.f.clone();
        Ok(Box::new(
            move |state| if assert(&*a) { Some(state) } else { None },
        ))
    }
}

impl<T: Unify> Goal for Assert1<T> {
    fn apply(&self, state: State) -> Option<State> {
        state.constrain(Rc::new(self.clone()))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        goals::{both::Both, unify::Unify},
        value::LVar,
    };

    use super::*;

    #[test]
    fn succeeds() {
        let x = LVar::new();
        let goal = Both::new(
            Unify::new(x.into(), Value::new(2)),
            Assert1::new(x.into(), move |x| *x > 1),
        );
        let result = goal.apply(State::new());
        assert_eq!(result.unwrap().resolve(&x.into()), Value::new(2));
    }

    #[test]
    fn fails() {
        let x = LVar::new();
        let goal = Both::new(
            Unify::new(x.into(), Value::new(1)),
            Assert1::new(x.into(), move |x| *x > 1),
        );
        let result = goal.apply(State::new());
        assert!(result.is_none());
    }
}
