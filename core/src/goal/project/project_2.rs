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

pub struct Project2<'a, A, B, D>
where
    A: Debug,
    B: Debug,
    D: DomainType<'a, A> + DomainType<'a, B>,
{
    a: Val<A>,
    b: Val<B>,
    f: Rc<dyn Fn(&A, &B) -> Goal<'a, D> + 'a>,
}

/// Create a [projection goal](super) that allows creating a new goal based on
/// the resolved values.
///
/// ```
/// use canrun::{Goal, all, unify, var, project_2};
/// use canrun::domains::example::I32;
///
/// let (x, y) = (var(), var());
/// let goal: Goal<I32> = all![
///     unify(1, x),
///     unify(2, y),
///     project_2(x, y, |x, y| if x < y { Goal::succeed() } else { Goal::fail() }),
/// ];
/// let result: Vec<_> = goal.query((x, y)).collect();
/// assert_eq!(result, vec![(1, 2)])
/// ```
pub fn project_2<'a, A, AV, B, BV, D, F>(a: AV, b: BV, func: F) -> Goal<'a, D>
where
    A: Debug + 'a,
    AV: IntoVal<A>,
    B: Debug + 'a,
    BV: IntoVal<B>,
    D: DomainType<'a, A> + DomainType<'a, B>,
    F: Fn(&A, &B) -> Goal<'a, D> + 'a,
{
    Goal(GoalEnum::Project(Rc::new(Project2 {
        a: a.into_val(),
        b: b.into_val(),
        f: Rc::new(func),
    })))
}

impl<'a, A, B, Dom> Project<'a, Dom> for Project2<'a, A, B, Dom>
where
    A: Debug,
    B: Debug,
    Dom: DomainType<'a, A> + DomainType<'a, B>,
{
    fn attempt<'r>(&'r self, state: State<'a, Dom>) -> Constraint<State<'a, Dom>> {
        let a = state.resolve_val(&self.a).clone();
        let b = state.resolve_val(&self.b).clone();
        match (a, b) {
            (Resolved(a), Resolved(b)) => Constraint::done({
                let goal = (*self.f)(&*a, &*b);
                goal.apply(state)
            }),
            (Var(var), _) => Constraint::on_1(state, var),
            (_, Var(var)) => Constraint::on_1(state, var),
        }
    }
}

impl<'a, A, B, D> Debug for Project2<'a, A, B, D>
where
    A: Debug,
    B: Debug,
    D: DomainType<'a, A> + DomainType<'a, B>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Project2 {:?} {:?}", self.a, self.b)
    }
}
