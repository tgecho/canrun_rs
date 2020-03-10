use super::Goal;
use crate::core::state::State;
use crate::core::state::Watch;
use crate::domain::Domain;
use std::fmt;
use std::rc::Rc;

#[derive(Clone)]
pub struct Project<'a, D: Domain<'a>>(Rc<dyn Fn(State<'a, D>) -> Watch<State<'_, D>> + 'a>);

impl<'a, D: Domain<'a>> Project<'a, D> {
    pub(crate) fn run(self, state: State<'a, D>) -> Option<State<'a, D>>
    where
        D: Domain<'a>,
    {
        let watch = self.0;
        state.watch(watch)
    }
}

pub fn project<'a, D, F>(func: F) -> Goal<'a, D>
where
    D: Domain<'a>,
    F: Fn(State<'a, D>) -> Watch<State<'a, D>> + 'a,
{
    Goal::Project(Project(Rc::new(func)))
}

impl<'a, D: Domain<'a>> fmt::Debug for Project<'a, D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Project ??")
    }
}

#[cfg(test)]
mod tests {
    use super::project;
    use super::Watch;
    use crate::core::tests::util;
    use crate::goal::unify::unify;
    use crate::value::{var, Val};

    #[test]
    fn succeeds() {
        let x = var();
        let goals = vec![
            unify(2, x),
            project(|s| match s.resolve_val(&Val::Var(x)).resolved() {
                Ok(x) => Watch::done(if x > &1 { Some(s) } else { None }),
                Err(x) => Watch::watch(s, x),
            }),
        ];
        util::all_permutations_resolve_to(goals, &x, vec![2]);
    }
}
