use super::Project;
use crate::domains::DomainType;
use crate::goal::{Goal, GoalEnum};
use crate::state::Constraint;
use crate::state::State;
use crate::value::{
    IntoVal, Val,
    Val::{Resolved, Var},
};
use std::fmt;
use std::fmt::Debug;
use std::rc::Rc;

pub struct Project1<'a, A: Debug, D: DomainType<'a, A>> {
    a: Val<A>,
    f: Rc<dyn Fn(&A) -> Goal<'a, D> + 'a>,
}

/// Create a [projection goal](super) that allows creating a new goal based on
/// the resolved value.
///
/// ```
/// use canrun::{Goal, both, unify, var, project_1};
/// use canrun::domains::example::I32;
///
/// let x = var();
/// let goal: Goal<I32> = both(unify(1, x), project_1(x, |x| if *x < 2 { Goal::succeed() } else { Goal::fail() }));
/// let result: Vec<_> = goal.query(x).collect();
/// assert_eq!(result, vec![1])
/// ```
pub fn project_1<'a, A, AV, D, F>(a: AV, func: F) -> Goal<'a, D>
where
    A: Debug + 'a,
    AV: IntoVal<A>,
    D: DomainType<'a, A>,
    F: Fn(&A) -> Goal<'a, D> + 'a,
{
    Goal(GoalEnum::Project(Rc::new(Project1 {
        a: a.into_val(),
        f: Rc::new(func),
    })))
}

impl<'a, A, Dom> Project<'a, Dom> for Project1<'a, A, Dom>
where
    A: Debug,
    Dom: DomainType<'a, A>,
{
    fn attempt<'r>(&'r self, state: State<'a, Dom>) -> Constraint<State<'a, Dom>> {
        let a = state.resolve_val(&self.a).clone();
        match a {
            Resolved(a) => Constraint::done({
                let goal = (*self.f)(&*a);
                goal.apply(state)
            }),
            Var(var) => Constraint::on_1(state, var),
        }
    }
}

impl<'a, A: Debug, D: DomainType<'a, A>> Debug for Project1<'a, A, D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Project1 {:?}", self.a)
    }
}
