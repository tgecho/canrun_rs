use super::resolved::ResolvedState;
use super::State;
use crate::domain::Domain;

pub type ResolvedIter<'s, D> = Box<dyn Iterator<Item = ResolvedState<'s, D>> + 's>;

pub trait IterResolved<'a, D: Domain<'a> + 'a> {
    fn resolved_iter(self) -> ResolvedIter<'a, D>;
}
impl<'a, D: Domain<'a> + 'a> IterResolved<'a, D> for State<'a, D> {
    fn resolved_iter(self) -> ResolvedIter<'a, D> {
        Box::new(self.iter_forks().map(|s| ResolvedState {
            domain: s.domain,
            watches: s.watches,
        }))
    }
}
impl<'a, D: Domain<'a> + 'a> IterResolved<'a, D> for Option<State<'a, D>> {
    fn resolved_iter(self) -> ResolvedIter<'a, D> {
        Box::new(self.into_iter().flat_map(|s| s.resolved_iter()))
    }
}
impl<'a, D: Domain<'a> + 'a> IterResolved<'a, D> for Vec<ResolvedState<'a, D>> {
    fn resolved_iter(self) -> ResolvedIter<'a, D> {
        Box::new(self.into_iter())
    }
}
