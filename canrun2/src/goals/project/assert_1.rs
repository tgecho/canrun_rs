use crate::core::Value;
use crate::core::{resolve_1, Constraint, ResolveFn, State, Unify, VarWatch};
use crate::goals::Goal;
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

pub fn assert_1<T, A, F>(a: A, func: F) -> Assert1<T>
where
    T: Unify,
    A: Into<Value<T>>,
    F: (Fn(&T) -> bool) + 'static,
{
    Assert1 {
        a: a.into(),
        f: Rc::new(func),
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
        Ok(Box::new(move |state| {
            if assert(a.as_ref()) {
                Some(state)
            } else {
                None
            }
        }))
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
        core::LVar,
        core::Query,
        goals::{both::both, unify::unify},
    };

    use super::*;

    #[test]
    fn succeeds() {
        let x = LVar::new();
        let goal = both(unify(x, 2), assert_1(x, move |x| *x > 1));
        assert_eq!(goal.query(x).collect::<Vec<_>>(), vec![2]);
    }

    #[test]
    fn fails() {
        let x = LVar::new();
        let goal = both(unify(x, 1), assert_1(x, move |x| *x > 1));
        assert_eq!(goal.query(x).count(), 0);
    }
}
