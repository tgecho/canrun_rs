pub mod assert_1;
pub mod assert_2;

use super::Goal;
use crate::domain::Domain;
use crate::state::State;
use crate::state::Watch;
use std::fmt;
use std::rc::Rc;

pub(crate) fn run<'a, D>(
    proj: Rc<dyn Project<'a, D> + 'a>,
    state: State<'a, D>,
) -> Option<State<'a, D>>
where
    D: Domain<'a>,
{
    state.watch(Rc::new(move |s| proj.attempt(s)))
}
pub trait Project<'a, D: Domain<'a>>: fmt::Debug {
    fn attempt<'r>(&'r self, state: State<'a, D>) -> Watch<State<'a, D>>;
}

#[cfg(test)]
mod tests {
    use super::assert_1::assert_1;
    use crate::domain::one::OfOne;
    use crate::goal::unify::unify;
    use crate::goal::Goal;
    use crate::tests::util;
    use crate::value::var;

    #[test]
    fn succeeds() {
        let x = var();
        let goals: Vec<Goal<OfOne<i32>>> = vec![unify(2, x), assert_1(x, |x| *x > 1)];
        util::all_permutations_resolve_to(goals, x, vec![2]);
    }
}
