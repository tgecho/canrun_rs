use crate::state::State;
use std::iter::once;

pub mod append;
pub mod both;
pub mod either;
pub mod equal;
pub mod lazy;

pub use both::both;
pub use either::either;
pub use equal::equal;
pub use lazy::lazy;

#[derive(Clone)]
pub enum Goal<T: Eq + Clone + 'static> {
    Equal(equal::EqualGoal<T>),
    Both(both::BothGoal<T>),
    Either(either::EitherGoal<T>),
    Lazy(lazy::LazyGoal<T>),
}

type GoalIter<T> = Box<dyn Iterator<Item = State<T>>>;

impl<T: Eq + Clone + 'static> Goal<T> {
    fn run<'a>(self, state: &'a State<T>) -> GoalIter<T> {
        match self {
            Goal::Equal(goal) => Box::new(state.unify(&goal.a, &goal.b).into_iter()) as GoalIter<T>,
            Goal::Both(goal) => Box::new(
                (goal.a.run(&state))
                    .zip(once(goal.b).cycle())
                    .flat_map(|(s, b)| b.run(&s)),
            ) as GoalIter<T>,
            Goal::Either(goal) => Box::new(goal.a.run(&state).chain(goal.b.run(&state))),
            Goal::Lazy(goal) => {
                let func = goal.0;
                func().run(state)
            }
        }
    }
}
