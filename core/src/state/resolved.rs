use super::State;
use crate::domains::{Domain, DomainType};
use crate::util::multikeymultivaluemap::MKMVMap;
use crate::value::{LVar, Val};

/// Derived from an open [State](crate::state::State), depending on
/// the constraints that have been applied.
///
/// Calling [.iter_resolved()](crate::IterResolved::iter_resolved()) is the
/// lowest level way to get an iterator of the possible resolved states, though
/// the [Query](crate::query::Query) interface is quite a bit nicer.
/// ```
/// use canrun::{State, ResolvedState, IterResolved, val, var};
/// use canrun::domains::example::I32;
///
/// let x = var();
///
/// let state = State::new()
///     .unify(&val!(x), &val!(1));
///
/// let results: Vec<_> = state.iter_resolved()
///     .map(|resolved: ResolvedState<I32>| resolved.get(x).ok().cloned())
///     .collect();
///
/// assert_eq!(results, vec![Some(1)]);
/// ```
#[derive(Clone)]
pub struct ResolvedState<D> {
    pub(super) domain: D,
}

impl<'a, D: Domain<'a> + 'a> ResolvedState<D> {
    fn resolve_val<'r, T>(&'r self, val: &'r Val<T>) -> &'r Val<T>
    where
        D: DomainType<'a, T>,
    {
        match val {
            Val::Var(var) => self.domain.values_as_ref().0.get(var).unwrap_or(val),
            value => value,
        }
    }

    pub fn get<'g, T>(&'g self, var: LVar<T>) -> Result<&'g T, LVar<T>>
    where
        D: DomainType<'a, T>,
    {
        match self.domain.values_as_ref().0.get(&var) {
            Some(val) => self.resolve_val(val).resolved(),
            None => Err(var),
        }
    }

    pub fn reopen(self) -> State<'a, D> {
        State {
            domain: self.domain,
            constraints: MKMVMap::new(),
            forks: im_rc::Vector::new(),
        }
    }
}
