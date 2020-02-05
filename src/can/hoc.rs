use crate::{pair, Can, CanT, LVar, ResolveResult, State, StateIter};
use im::HashSet;
use std::iter::{empty, once};

pub type HocUnifyFn<T> = fn(LVar, Can<T>, Can<T>, State<T>) -> StateIter<T>;

#[derive(Clone)]
pub struct Hoc<T: CanT> {
    pub composed: bool,
    pub var: LVar,
    pub value: Box<Can<T>>,
    pub unify: HocUnifyFn<T>,
}

impl<T: CanT + 'static> Hoc<T> {
    pub(crate) fn resolve_in(&self, state: &State<T>, history: &HashSet<LVar>) -> ResolveResult<T> {
        if state.contains_var(&self.var) {
            state.checked_resolve(&self.var.can(), history)
        } else if self.composed {
            Ok(Can::Hoc(self.clone()))
        } else {
            Ok(Can::Hoc(Hoc {
                composed: false,
                var: self.var.clone(),
                value: Box::new(state.checked_resolve(&self.value, history)?),
                unify: self.unify,
            }))
        }
    }

    pub(crate) fn unify_with(self, other: Can<T>, state: &State<T>) -> StateIter<T> {
        match other.clone() {
            Can::Hoc(Hoc { var, .. }) => {
                let combined = Can::Hoc(Hoc {
                    composed: true,
                    var: LVar::new(),
                    value: Box::new(pair(Can::Hoc(self.clone()), other.clone())),
                    unify: unify_combined,
                });
                // by definition we only arrive here if both self and other are unresolved, so we
                // can just assign directly to avoid resolving the contents
                Box::new(once(
                    state
                        .assign(self.var, combined.clone())
                        .assign(var, combined),
                ))
            }
            other => {
                let unify = self.unify;
                unify(self.var, *self.value, other, state.clone())
            }
        }
    }
}

fn unify_combined<T: CanT + 'static>(
    var: LVar,
    value: Can<T>,
    other: Can<T>,
    state: State<T>,
) -> StateIter<T> {
    match value {
        Can::Pair { l, r } => match (*l, *r) {
            (Can::Hoc(l), Can::Hoc(r)) => {
                let l_unify = l.unify;
                let r_unify = r.unify;
                // The trick here is that we want to apply the inner hocs WITHOUT resolving their .var fields, which would loop back to the composed hoc
                // this clearly exposes the shoddiness of the overall data structure, but I think we've almost got something here.
                // needs more massaging and boiling down to find the hopefully elegant core in this small mess
                let state = state.assign(var, other.clone());
                let iter = l_unify(l.var, *l.value, other.clone(), state)
                    .zip(once(r).cycle())
                    .flat_map(move |(s, r)| r_unify(r.var, *r.value, other.clone(), s));
                Box::new(iter)
            }
            _ => Box::new(empty()),
        },
        _ => Box::new(empty()),
    }
}
