use crate::{Domain, DomainType, ReifyIn, ResolvedState, State, UnifyIn};
use std::rc::Rc;

macro_rules! impl_unify_eq {
    ($($type:ty),+) => {
        $(
            impl <'a, D: DomainType<'a, $type>> UnifyIn<'a, D> for $type {
                fn unify_resolved(state: State<'a, D>, a: Rc<$type>, b: Rc<$type>) -> Option<State<'a, D>> {
                    if a == b {
                        Some(state)
                    } else {
                        None
                    }
                }
            }
        )+
    };
}

macro_rules! impl_reify_copy {
    ($($type:ty),+) => {
        $(
            impl<'a, D: Domain<'a>> ReifyIn<'a, D> for $type {
                type Reified = $type;
                fn reify_in(&self, _: &ResolvedState<D>) -> Option<$type> {
                    Some(*self)
                }
            }
        )+
    }
}

macro_rules! impl_reify_clone {
    ($($type:ty),+) => {
        $(
            impl<'a, D: Domain<'a>> ReifyIn<'a, D> for $type {
                type Reified = $type;
                fn reify_in(&self, _: &ResolvedState<D>) -> Option<$type> {
                    Some(self.clone())
                }
            }
        )+
    }
}

impl_unify_eq!(i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64);
impl_unify_eq!(String, &'static str, bool, char);
impl_unify_eq!(std::ops::Range<usize>);

impl_reify_copy!(i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64);
impl_reify_clone!(String);
impl_reify_copy!(&'static str, bool, char);

#[cfg(test)]
mod tests {
    use crate::query::Query;
    use crate::{val, var, State};

    canrun_codegen::canrun_internal_domain! {
        pub Strings { String }
    }

    #[test]
    fn partial_eq() {
        let a = var();
        let value = val!("foo".to_string());
        let state: Option<State<Strings>> = State::new().unify(&val!(a), &value);
        let result: Vec<_> = state.query(a).collect();
        assert_eq!(result, vec!["foo".to_string()]);
    }
}
