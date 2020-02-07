use crate::{Can, CanT, Goal, LVar, ResolveResult, State, StateIter};
use im::HashSet;
use std::iter::once;

pub type HocUnifyFn<T> = fn(LVar, Can<T>, Can<T>) -> Goal<T>;

pub fn hoc_fn<T: CanT>(input: Can<T>, unify: HocUnifyFn<T>) -> Can<T> {
    Can::Hoc(Hoc {
        output: LVar::new(),
        body: HocBody::Fn {
            input: Box::new(input),
            unify: unify,
        },
    })
}

#[derive(Clone)]
enum HocBody<T: CanT> {
    Fn {
        input: Box<Can<T>>,
        unify: HocUnifyFn<T>,
    },
    Pair {
        a: Box<Hoc<T>>,
        b: Box<Hoc<T>>,
    },
}

#[derive(Clone)]
pub struct Hoc<T: CanT> {
    output: LVar,
    body: HocBody<T>,
}

impl<'a, T: CanT + 'a> Hoc<T> {
    pub(crate) fn resolve_in(&self, state: &State<T>, history: &HashSet<LVar>) -> ResolveResult<T> {
        match state.checked_resolve(&self.output.can(), history)? {
            Can::Var(var) if var == self.output => match &self.body {
                HocBody::Fn { input, unify } => Ok(Can::Hoc(Hoc {
                    output: self.output,
                    body: HocBody::Fn {
                        input: Box::new(state.checked_resolve(&input, history)?),
                        unify: *unify,
                    },
                })),
                HocBody::Pair { .. } => Ok(Can::Hoc(self.clone())),
            },
            resolved => Ok(resolved),
        }
    }

    pub(crate) fn unify_with(self, other: Can<T>, state: State<T>) -> StateIter<'a, T> {
        match other.clone() {
            Can::Hoc(other_hoc) => {
                let self_out = self.output;
                let other_out = other_hoc.output;
                let combined = Can::Hoc(Hoc {
                    output: LVar::new(),
                    body: HocBody::Pair {
                        a: Box::new(self),
                        b: Box::new(other_hoc),
                    },
                });
                // by definition we only arrive here if both self and other are unresolved, so we
                // can just assign directly to avoid further resolving the contents
                Box::new(once(
                    state
                        .assign(self_out, combined.clone())
                        .assign(other_out, combined),
                ))
            }
            other_can => match self.body {
                HocBody::Fn { input, unify } => unify(self.output, *input, other_can).run(state),
                HocBody::Pair { a, b } => {
                    let state = state.assign(self.output, other.clone());
                    let iter = a
                        .unify_with(other.clone(), state)
                        .zip(once(b).cycle())
                        .flat_map(move |(s, b)| b.unify_with(other.clone(), s));
                    Box::new(iter)
                }
            },
        }
    }
}
