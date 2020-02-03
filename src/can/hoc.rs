use crate::{Can, CanT, LVar, ResolveResult, State, StateIter};
use im::HashSet;

pub type HocUnifyFn<T> = fn(LVar, Can<T>, Can<T>, State<T>) -> StateIter<T>;

pub(crate) fn resolve<T: CanT + 'static>(
    state: &State<T>,
    var: &LVar,
    value: &Box<Can<T>>,
    unify: &HocUnifyFn<T>,
    history: &HashSet<LVar>,
) -> ResolveResult<T> {
    if state.contains_var(var) {
        state.checked_resolve(&var.can(), history)
    } else {
        Ok(Can::HoC {
            var: var.clone(),
            value: Box::new(state.checked_resolve(value, history)?),
            unify: *unify,
        })
    }
}
