use crate::goals::Goal;
use crate::{
    constraints::{resolve_2, Constraint, ResolveFn, VarWatch},
    core::{State, Unify, Value},
};
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

pub fn assert_2<A, IA, B, IB, F>(a: IA, b: IB, func: F) -> Assert2<A, B>
where
    A: Unify,
    IA: Into<Value<A>>,
    B: Unify,
    IB: Into<Value<B>>,
    F: (Fn(&A, &B) -> bool) + 'static,
{
    Assert2 {
        a: a.into(),
        b: b.into(),
        f: Rc::new(func),
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
        core::{LVar, Query},
        goals::{both::both, unify},
    };

    use super::*;

    #[test]
    fn succeeds() {
        let x = LVar::new();
        let goal = both(unify(x, 2), assert_2(x, 1, move |x, y| x > y));
        assert_eq!(goal.query(x).collect::<Vec<_>>(), vec![2]);
    }

    #[test]
    fn fails() {
        let x = LVar::new();
        let goal = both(unify(x, 1), assert_2(x, 1, move |x, y| x > y));
        assert_eq!(goal.query(x).count(), 0);
    }
}
