use crate::core::state::State;
use crate::domain::Domain;

mod all;
mod any;
mod both;
mod custom;
mod either;
mod lazy;
mod not;
mod project;
mod unify;
pub use all::all;
pub use any::any;
pub use both::both;
pub use custom::custom;
pub use either::either;
pub use lazy::lazy;
pub use not::not;
pub use project::project;
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
    Project(project::Project<'a, D>),
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
            Goal::Project(project) => project.run(state),
        }
    }
}
