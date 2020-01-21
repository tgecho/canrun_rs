use crate::state::State;
pub mod equal;
pub mod with;
pub trait Goal<T>
where
    T: Eq + Clone,
{
    fn run<'a>(self, state: &'a State<T>) -> Box<dyn Iterator<Item = State<T>> + 'a>;
}
