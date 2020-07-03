use crate::domains::DomainType;
use crate::goal::Goal;
use crate::state::constraints::{resolve_2, Constraint, ResolveFn, VarWatch};
use crate::state::State;
use crate::value::{IntoVal, Val};
use std::fmt;
use std::fmt::Debug;
use std::rc::Rc;

pub struct Assert2<'a, A: Debug, B: Debug> {
    a: Val<A>,
    b: Val<B>,
    f: Rc<dyn Fn(&A, &B) -> bool + 'a>,
}

/// Create a [projection goal](super) that succeeds if the resolved values pass
/// an assertion test.
///
/// ```
/// use canrun::{Goal, all, unify, var, assert_2};
/// use canrun::example::I32;
///
/// let (x, y) = (var(), var());
/// let goal: Goal<I32> = all![
///     unify(1, x),
///     unify(2, y),
///     assert_2(x, y, |x, y| x < y),
/// ];
/// let result: Vec<_> = goal.query((x, y)).collect();
/// assert_eq!(result, vec![(1, 2)])
/// ```
pub fn assert_2<'a, A, AV, B, BV, D, F>(a: AV, b: BV, func: F) -> Goal<'a, D>
where
    A: Debug + 'a,
    AV: IntoVal<A>,
    B: Debug + 'a,
    BV: IntoVal<B>,
    D: DomainType<'a, A> + DomainType<'a, B>,
    F: Fn(&A, &B) -> bool + 'a,
{
    Goal::constraint(Assert2 {
        a: a.into_val(),
        b: b.into_val(),
        f: Rc::new(func),
    })
}

impl<'a, A, B, Dom> Constraint<'a, Dom> for Assert2<'a, A, B>
where
    A: Debug + 'a,
    B: Debug + 'a,
    Dom: DomainType<'a, A> + DomainType<'a, B>,
{
    fn attempt(&self, state: &State<'a, Dom>) -> Result<ResolveFn<'a, Dom>, VarWatch> {
        let (a, b) = resolve_2(&self.a, &self.b, state)?;
        let assert = self.f.clone();
        Ok(Box::new(
            move |state| if assert(&*a, &*b) { Some(state) } else { None },
        ))
    }
}

impl<'a, A: Debug, B: Debug> Debug for Assert2<'a, A, B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Assert2 {:?} {:?}", self.a, self.b)
    }
}
