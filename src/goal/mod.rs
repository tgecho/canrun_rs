use crate::state::State;
pub mod both;
pub mod either;
pub mod equal;
pub mod with;

pub trait Goal<T>: Clone
where
    T: Eq + Clone,
{
    fn run<'a>(self, state: &'a State<T>) -> Box<dyn Iterator<Item = State<T>> + 'a>;
}
