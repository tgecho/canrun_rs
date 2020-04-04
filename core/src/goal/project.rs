mod assert_1;
mod assert_2;
mod map_1;
mod map_2;

#[doc(inline)]
pub use assert_1::assert_1;
#[doc(inline)]
pub use assert_2::assert_2;
#[doc(inline)]
pub use map_1::map_1;
#[doc(inline)]
pub use map_2::map_2;

use super::Goal;
use crate::domains::Domain;
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
    use crate::domains::example::I32;
    use crate::goal::unify::unify;
    use crate::goal::Goal;
    use crate::util;
    use crate::value::var;

    #[test]
    fn succeeds() {
        let x = var();
        let goals: Vec<Goal<I32>> = vec![unify(2, x), assert_1(x, |x| *x > 1)];
        util::all_permutations_resolve_to(goals, x, vec![2]);
    }
}
