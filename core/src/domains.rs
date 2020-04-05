use crate::state::State;
use crate::value::{LVar, Val};
use im_rc::HashMap;
use std::fmt::Debug;

pub mod example {
    use crate::value::Val;

    canrun_codegen::canrun_internal_domains! {
        pub domain I32 { i32 }
        pub domain VecI32 {
            i32,
            Vec<Val<i32>>,
        }
        pub domain TupleI32 {
            i32,
            (Val<i32>, Val<i32>),
        }
    }
}

pub trait Domain<'a>: Clone + Debug {
    type Value: Debug + Clone + 'a;
    fn new() -> Self;
    fn unify_domain_values(
        state: State<'a, Self>,
        a: Self::Value,
        b: Self::Value,
    ) -> Option<State<'a, Self>>;
}

pub trait DomainType<'a, T>: Domain<'a> {
    fn values_as_ref(&self) -> &HashMap<LVar<T>, Val<T>>;
    fn values_as_mut(&mut self) -> &mut HashMap<LVar<T>, Val<T>>;
}

pub trait IntoDomainVal<'a, T>: Domain<'a> {
    fn into_domain_val(val: Val<T>) -> Self::Value;
}
