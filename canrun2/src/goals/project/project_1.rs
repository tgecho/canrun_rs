use std::fmt;
use std::fmt::Debug;
use std::rc::Rc;

use crate::goals::Goal;
use crate::{
    constraints::{resolve_1, Constraint, ResolveFn, VarWatch},
    core::{State, Unify, Value},
};

#[allow(clippy::type_complexity)]
pub struct Project1<A: Unify> {
    a: Value<A>,
    f: Rc<dyn Fn(&A) -> Box<dyn Goal>>,
}

pub fn project_1<A, IA, F>(a: IA, func: F) -> Project1<A>
where
    A: Unify,
    IA: Into<Value<A>>,
    F: Fn(&A) -> Box<dyn Goal> + 'static,
{
    Project1 {
        a: a.into(),
        f: Rc::new(func),
    }
}

impl<A: Unify> Clone for Project1<A> {
    fn clone(&self) -> Self {
        Self {
            a: self.a.clone(),
            f: self.f.clone(),
        }
    }
}

impl<A: Unify> Goal for Project1<A> {
    fn apply(&self, state: State) -> Option<State> {
        state.constrain(Rc::new(self.clone()))
    }
}

impl<A: Unify> Constraint for Project1<A> {
    fn attempt(&self, state: &State) -> Result<ResolveFn, VarWatch> {
        let a = resolve_1(&self.a, state)?;
        let goal = (self.f)(&*a);
        Ok(Box::new(move |state| goal.apply(state)))
    }
}

impl<A: Unify + Debug> Debug for Project1<A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Project1 {:?}", self.a)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        core::{LVar, Query},
        goals::{both::both, fail::Fail, project::project_1::project_1, succeed::Succeed, unify},
    };

    #[test]
    fn succeeds() {
        let x = LVar::new();
        let goal = both(
            unify(1, x),
            project_1(x, |x| {
                if *x < 2 {
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
        let goal = both(
            unify(1, x),
            project_1(x, |x| {
                if *x < 1 {
                    Box::new(Succeed)
                } else {
                    Box::new(Fail)
                }
            }),
        );
        assert_eq!(goal.query(x).collect::<Vec<_>>(), vec![]);
    }
}
