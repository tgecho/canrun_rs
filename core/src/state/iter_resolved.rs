use super::resolved::ResolvedState;
use super::State;
use crate::domains::Domain;

pub type ResolvedIter<'s, D> = Box<dyn Iterator<Item = ResolvedState<D>> + 's>;

pub trait IterResolved<'a, D: Domain<'a> + 'a> {
    fn iter_resolved(self) -> ResolvedIter<'a, D>;
}

impl<'a, D: Domain<'a> + 'a> IterResolved<'a, D> for State<'a, D> {
    fn iter_resolved(self) -> ResolvedIter<'a, D> {
        Box::new(self.iter_forks().filter_map(|s: State<'a, D>| {
            if s.constraints.is_empty() {
                Some(ResolvedState { domain: s.domain })
            } else {
                None
            }
        }))
    }
}

impl<'a, D: Domain<'a> + 'a> IterResolved<'a, D> for Option<State<'a, D>> {
    fn iter_resolved(self) -> ResolvedIter<'a, D> {
        Box::new(self.into_iter().flat_map(State::iter_resolved))
    }
}
