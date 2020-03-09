use super::domain::Domain;
use super::state::State;

pub mod all;
pub mod any;
pub mod both;
pub mod custom;
pub mod either;
pub mod lazy;
pub mod not;
pub mod project;
pub mod unify;

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
    pub(crate) fn apply(self, state: State<'a, D>) -> Option<State<'a, D>> {
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
