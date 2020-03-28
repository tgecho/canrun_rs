use crate::domain::Domain;
use crate::state::State;
use std::rc::Rc;

mod all;
mod any;
mod both;
mod custom;
mod either;
mod lazy;
mod not;
pub mod project;
mod unify;
pub use all::all;
pub use any::any;
pub use both::both;
pub use custom::custom;
pub use either::either;
pub use lazy::lazy;
pub use not::not;
pub use project::assert_1::assert_1;
pub use project::assert_2::assert_2;
pub use project::map_1::map_1;
pub use project::map_2::map_2;
pub use unify::unify;

#[derive(Clone, Debug)]
pub enum Goal<'a, D: Domain<'a>> {
    Unify(D::Value, D::Value),
    Both(Box<Goal<'a, D>>, Box<Goal<'a, D>>),
    All(Vec<Goal<'a, D>>),
    Either(Box<Goal<'a, D>>, Box<Goal<'a, D>>),
    Any(Vec<Goal<'a, D>>),
    Not(Box<Goal<'a, D>>),
    Lazy(lazy::Lazy<'a, D>),
    Custom(custom::Custom<'a, D>),
    Project(Rc<dyn project::Project<'a, D> + 'a>),
}

impl<'a, D: Domain<'a> + 'a> Goal<'a, D> {
    pub fn apply(self, state: State<'a, D>) -> Option<State<'a, D>> {
        match self {
            Goal::Unify(a, b) => unify::run(state, a, b),
            Goal::Both(a, b) => both::run(state, *a, *b),
            Goal::All(goals) => all::run(state, goals),
            Goal::Either(a, b) => either::run(state, *a, *b),
            Goal::Any(goals) => any::run(state, goals),
            Goal::Not(goal) => not::run(state, *goal),
            Goal::Lazy(lazy) => lazy.run(state),
            Goal::Custom(custom) => custom.run(state),
            Goal::Project(proj) => project::run(proj, state),
        }
    }
}
