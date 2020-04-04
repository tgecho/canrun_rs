use crate::domains::Domain;
use crate::state::State;
use std::rc::Rc;

mod all;
mod any;
mod both;
mod custom;
mod either;
mod lazy;
pub mod project;
mod unify;

#[doc(inline)]
pub use all::all;
#[doc(inline)]
pub use any::any;
#[doc(inline)]
pub use both::both;
#[doc(inline)]
pub use custom::custom;
#[doc(inline)]
pub use either::either;
#[doc(inline)]
pub use lazy::lazy;
#[doc(inline)]
pub use unify::unify;

#[derive(Clone, Debug)]
pub enum Goal<'a, D: Domain<'a>> {
    Unify(D::Value, D::Value),
    Both(Box<Goal<'a, D>>, Box<Goal<'a, D>>),
    All(Vec<Goal<'a, D>>),
    Either(Box<Goal<'a, D>>, Box<Goal<'a, D>>),
    Any(Vec<Goal<'a, D>>),
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
            Goal::Lazy(lazy) => lazy.run(state),
            Goal::Custom(custom) => custom.run(state),
            Goal::Project(proj) => project::run(proj, state),
        }
    }
}
