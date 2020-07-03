use crate::domains::DomainType;
use crate::example::I32;
use crate::goal::custom;
use crate::goal::unify;
use crate::goal::Goal;
use crate::state::constraints::{Constraint, ResolveFn, VarWatch};
use crate::state::State;
use crate::util;
use crate::value::{
    val, var, IntoVal, Val,
    Val::{Resolved, Var},
};
use std::fmt;
use std::fmt::Debug;
use std::rc::Rc;

struct Assert<'a, T: Debug> {
    val: Val<T>,
    assert: Rc<dyn Fn(&T) -> bool + 'a>,
}

impl<'a, T: fmt::Debug> fmt::Debug for Assert<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Assert({:?})", self.val)
    }
}

impl<'a, T, D> Constraint<'a, D> for Assert<'a, T>
where
    T: Debug + 'a,
    D: DomainType<'a, T>,
{
    fn attempt(&self, state: &State<'a, D>) -> Result<ResolveFn<'a, D>, VarWatch> {
        let resolved = state.resolve_val(&self.val).clone();
        match resolved {
            Resolved(val) => {
                let assert = self.assert.clone();
                Ok(Box::new(
                    move |state: State<'a, D>| if assert(&*val) { Some(state) } else { None },
                ))
            }
            Var(var) => Err(VarWatch::one(var)),
        }
    }
}

pub(crate) fn assert<'a, T, V, D, F>(val: V, func: F) -> Rc<dyn Constraint<'a, D> + 'a>
where
    T: Debug + 'a,
    V: IntoVal<T> + Clone + 'a,
    D: DomainType<'a, T> + 'a,
    F: Fn(&T) -> bool + 'a,
{
    Rc::new(Assert {
        val: val.into_val(),
        assert: Rc::new(func),
    })
}

#[test]
fn basic_constrain_succeeds() {
    let x = var();
    let goals: Vec<Goal<I32>> = vec![
        unify(2, x),
        custom(|s| s.constrain(assert(x, |x| x > &1))),
        custom(|s| s.constrain(assert(x, |x| x > &0))),
    ];
    util::assert_permutations_resolve_to(goals, x, vec![2]);
}

#[test]
fn basic_constrain_fails() {
    let x = var();
    let goals: Vec<Goal<I32>> = vec![
        unify(&val!(2), x.clone()),
        custom(|s| s.constrain(assert(x.clone(), |x| x > &1))),
        custom(|s| s.constrain(assert(x, |x| x > &3))),
    ];
    util::assert_permutations_resolve_to(goals, x, vec![]);
}

#[test]
fn unsatisfied_constrain_fails() {
    let x = var();
    let y = var();
    let goals: Vec<Goal<I32>> = vec![
        unify(&val!(1), x),
        custom(|s| s.constrain(assert(y, |y| y < &3))),
    ];
    util::assert_permutations_resolve_to(goals.clone(), x, vec![]);
    util::assert_permutations_resolve_to(goals, y, vec![]);
}
