use super::{Domain, DomainType, IntoDomainVal, Unified, UnifyIn};
use crate::state::State;
use crate::value::{LVar, Val};
use im::HashMap;
use std::fmt::Debug;

type T1 = i32;
type T2 = Vec<i32>;

macro_rules! impl_into_domain_val {
    ($type:ty, $variant:path, $domain_value:ident) => {
        impl<'a> IntoDomainVal<'a, $type> for OfTwo {
            fn into_domain_val(val: Val<$type>) -> $domain_value {
                $variant(val)
            }
        }
    };
}

macro_rules! impl_domain_type {
    ($domain:ty, $type:ty, $property:ident) => {
        impl<'a> DomainType<'a, $type> for $domain {
            fn values_as_ref(&self) -> &HashMap<LVar<$type>, Val<$type>> {
                &self.$property
            }
            fn values_as_mut(&mut self) -> &mut HashMap<LVar<$type>, Val<$type>> {
                &mut self.$property
            }
        }
    };
}

macro_rules! impl_unify_eq {
    ($domain:ty, $type:ty) => {
        impl<'a> UnifyIn<'a, OfTwo> for $type {
            fn unify_with(&self, other: &Self) -> Unified<'a, OfTwo> {
                if self == other {
                    Unified::Success
                } else {
                    Unified::Failed
                }
            }
        }
    };
}

macro_rules! create_types {
    ($domain:ident, $($name:ident:$type:ty),+) => {

        #[derive(Debug)]
        pub struct $domain {
            $($name: HashMap<LVar<$type>, Val<$type>>),+
        }

        #[allow(non_camel_case_types)]
        #[derive(Debug)]
        pub enum DomainValue {
            $($name(Val<$type>)),+
        }

        impl<'a> Clone for $domain {
            fn clone(&self) -> Self {
                $domain {
                    $($name: self.$name.clone()),+
                }
            }
        }

        impl Clone for DomainValue {
            fn clone(&self) -> Self {
                match self {
                    $(DomainValue::$name(val) => DomainValue::$name(val.clone())),+
                }
            }
        }

        impl<'a> Domain<'a> for $domain {
            type Value = DomainValue;
            fn new() -> Self {
                $domain {
                    $($name: HashMap::new(),)+
                }
            }
            fn unify_domain_values(
                state: State<'a, Self>,
                a: Self::Value,
                b: Self::Value,
            ) -> Option<State<Self>> {
                match (a, b) {
                    $((DomainValue::$name(a), DomainValue::$name(b)) => state.unify::<$type, Val<$type>, Val<$type>>(a, b),)+
                    _ => None, // This should only happen if a DomainVal constructor allows two values with different types.
                }
            }
        }

        $(impl_into_domain_val!($type, DomainValue::$name, DomainValue);)+
        $(impl_domain_type!($domain, $type, $name);)+
        $(impl_unify_eq!($domain, $type);)+
    };
}

create_types!(OfTwo, t1: T1, t2: T2);

#[cfg(test)]
mod tests {
    use super::OfTwo;
    use crate::goal::{all, project, unify, Goal};
    use crate::state::{State, Watch};
    use crate::tests::util;
    use crate::value::{var, Val};

    #[test]
    fn succeeds() {
        let x = var::<Vec<i32>>();
        let y = var::<i32>();
        let goal: Goal<OfTwo> = all::<OfTwo>(vec![
            unify(x, vec![1, 2, 3]),
            unify(y, 1),
            project(|s: State<OfTwo>| {
                // This is pretty gnarly
                let x = Val::Var(x);
                let x = s.resolve_val(&x).resolved();
                let y = Val::Var(y);
                let y = s.resolve_val(&y).resolved();
                match (x, y) {
                    (Ok(x), Ok(y)) => Watch::done(if x.contains(y) { Some(s) } else { None }),
                    (Err(x), Err(y)) => Watch::watch(s, x).and(y),
                    (_, Err(y)) => Watch::watch(s, y),
                    (Err(x), _) => Watch::watch(s, x),
                }
            }) as Goal<OfTwo>,
        ]);
        let result = util::goal_resolves_to(goal, (&x, &y));
        assert_eq!(result, vec![(vec![1, 2, 3], 1)]);
    }
}
