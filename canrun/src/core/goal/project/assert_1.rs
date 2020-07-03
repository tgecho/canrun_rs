use crate::domains::DomainType;
use crate::goal::Goal;
use crate::state::constraints::{resolve_1, Constraint, ResolveFn, VarWatch};
use crate::state::State;
use crate::value::{IntoVal, Val};
use std::fmt;
use std::fmt::Debug;
use std::rc::Rc;

pub struct Assert1<'a, A: Debug> {
    a: Val<A>,
    f: Rc<dyn Fn(&A) -> bool + 'a>,
}

/// Create a [projection goal](super) that succeeds if the resolved value passes
/// an assertion test.
///
/// ```
/// use canrun::{Goal, both, unify, var, assert_1};
/// use canrun::example::I32;
///
/// let x = var();
/// let goal: Goal<I32> = both(unify(1, x), assert_1(x, |x| *x < 2));
/// let result: Vec<_> = goal.query(x).collect();
/// assert_eq!(result, vec![1])
/// ```
pub fn assert_1<'a, A, AV, D, F>(a: AV, func: F) -> Goal<'a, D>
where
    A: Debug + 'a,
    AV: IntoVal<A>,
    D: DomainType<'a, A>,
    F: Fn(&A) -> bool + 'a,
{
    Goal::constraint(Assert1 {
        a: a.into_val(),
        f: Rc::new(func),
    })
}

impl<'a, A, Dom> Constraint<'a, Dom> for Assert1<'a, A>
where
    A: Debug + 'a,
    Dom: DomainType<'a, A>,
{
    fn attempt(&self, state: &State<'a, Dom>) -> Result<ResolveFn<'a, Dom>, VarWatch> {
        let a = resolve_1(&self.a, state)?;
        let assert = self.f.clone();
        Ok(Box::new(
            move |state| if assert(&*a) { Some(state) } else { None },
        ))
    }
}

impl<'a, A: Debug> Debug for Assert1<'a, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Assert1 {:?}", self.a)
    }
}
