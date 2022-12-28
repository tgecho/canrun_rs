use std::fmt;
use std::fmt::Debug;
use std::rc::Rc;

use crate::core::{resolve_2, Constraint, ResolveFn, State, Unify, VarWatch};
use crate::goals::Goal;
use crate::value::Value;

#[allow(clippy::type_complexity)]
pub struct Project2<A: Unify, B: Unify> {
    a: Value<A>,
    b: Value<B>,
    f: Rc<dyn Fn(&A, &B) -> Box<dyn Goal>>,
}

pub fn project_2<A, IA, B, IB, F>(a: IA, b: IB, func: F) -> Project2<A, B>
where
    A: Unify,
    IA: Into<Value<A>>,
    B: Unify,
    IB: Into<Value<B>>,
    F: Fn(&A, &B) -> Box<dyn Goal> + 'static,
{
    Project2 {
        a: a.into(),
        b: b.into(),
        f: Rc::new(func),
    }
}

impl<A: Unify, B: Unify> Clone for Project2<A, B> {
    fn clone(&self) -> Self {
        Self {
            a: self.a.clone(),
            b: self.b.clone(),
            f: self.f.clone(),
        }
    }
}

impl<A: Unify, B: Unify> Goal for Project2<A, B> {
    fn apply(&self, state: State) -> Option<State> {
        state.constrain(Rc::new(self.clone()))
    }
}

impl<A: Unify, B: Unify> Constraint for Project2<A, B> {
    fn attempt(&self, state: &State) -> Result<ResolveFn, VarWatch> {
        let (a, b) = resolve_2(&self.a, &self.b, state)?;
        let goal = (self.f)(&*a, &*b);
        Ok(Box::new(move |state| goal.apply(state)))
    }
}

impl<A: Unify + Debug, B: Unify + Debug> Debug for Project2<A, B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Project2 {:?} {:?}", self.a, self.b)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        core::Query,
        goals::{
            both::both, fail::Fail, project::project_2::project_2, succeed::Succeed, unify::unify,
        },
        value::LVar,
    };

    #[test]
    fn succeeds() {
        let x = LVar::new();
        let y = LVar::new();
        let goal = both(
            both(unify(1, x), unify(2, y)),
            project_2(x, y, |x, y| {
                if x < y {
                    Box::new(Succeed)
                } else {
                    Box::new(Fail)
                }
            }),
        );
        assert_eq!(goal.query(x).collect::<Vec<_>>(), vec![1]);
    }

    #[test]
    fn fails() {
        let x = LVar::new();
        let y = LVar::new();
        let goal = both(
            both(unify(1, x), unify(2, y)),
            project_2(x, y, |x, y| {
                if x > y {
                    Box::new(Succeed)
                } else {
                    Box::new(Fail)
                }
            }),
        );
        assert_eq!(goal.query(x).collect::<Vec<_>>(), vec![]);
    }
}
