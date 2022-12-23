use super::State;
use crate::value::VarId;

pub type ResolveFn = Box<dyn FnOnce(State) -> Option<State>>;

#[derive(Debug)]
pub struct VarWatch(pub(crate) Vec<VarId>);

pub trait Constraint {
    /// Resolve required variables in a state and resubscribe or request to
    /// update the state.
    fn attempt(&self, state: &State) -> Result<ResolveFn, VarWatch>;
}
