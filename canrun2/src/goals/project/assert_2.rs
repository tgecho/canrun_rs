use crate::core::{resolve_2, Constraint, ResolveFn, State, Unify, VarWatch};
use crate::goals::Goal;
use crate::value::Value;
use std::fmt::{self, Debug};
use std::rc::Rc;

pub struct Assert2<A: Unify, B: Unify> {
    a: Value<A>,
    b: Value<B>,
    #[allow(clippy::type_complexity)]
    f: Rc<dyn Fn(&A, &B) -> bool>,
}

impl<A: Unify, B: Unify> Clone for Assert2<A, B> {
    fn clone(&self) -> Self {
        Self {
            a: self.a.clone(),
            b: self.b.clone(),
            f: self.f.clone(),
        }
    }
}

impl<A: Unify, B: Unify> Assert2<A, B> {
    pub fn new<F>(a: Value<A>, b: Value<B>, func: F) -> Self
    where
        F: (Fn(&A, &B) -> bool) + 'static,
    {
        Assert2 {
            a,
            b,
            f: Rc::new(func),
        }
    }
}

impl<A: Unify, B: Unify> Debug for Assert2<A, B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Assert2 {:?} {:?}", self.a, self.b)
    }
}

impl<A: Unify, B: Unify> Constraint for Assert2<A, B> {
    fn attempt(&self, state: &State) -> Result<ResolveFn, VarWatch> {
        let (a, b) = resolve_2(&self.a, &self.b, state)?;
        let assert = self.f.clone();
        Ok(Box::new(move |state| {
            if assert(a.as_ref(), b.as_ref()) {
                Some(state)
            } else {
                None
            }
        }))
    }
}

impl<A: Unify, B: Unify> Goal for Assert2<A, B> {
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
            Assert2::new(x.into(), 1.into(), move |x, y| x > y),
        );
        let result = goal.apply(State::new());
        assert_eq!(result.unwrap().resolve(&x.into()), Value::new(2));
    }

    #[test]
    fn fails() {
        let x = LVar::new();
        let goal = Both::new(
            Unify::new(x.into(), Value::new(1)),
            Assert2::new(x.into(), 1.into(), move |x, y| x > y),
        );
        let result = goal.apply(State::new());
        assert!(result.is_none());
    }
}
