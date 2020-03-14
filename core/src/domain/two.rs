macro_rules! impl_into_domain_val {
    ($type:ty, $variant:path, $domain_value:ident) => {
        impl<'a> crate::domain::IntoDomainVal<'a, $type> for OfTwo {
            fn into_domain_val(val: crate::value::Val<$type>) -> $domain_value {
                $variant(val)
            }
        }
    };
}

macro_rules! impl_domain_type {
    ($domain:ty, $type:ty, $property:ident) => {
        impl<'a> crate::domain::DomainType<'a, $type> for $domain {
            fn values_as_ref(
                &self,
            ) -> &im::HashMap<crate::value::LVar<$type>, crate::value::Val<$type>> {
                &self.$property
            }
            fn values_as_mut(
                &mut self,
            ) -> &mut im::HashMap<crate::value::LVar<$type>, crate::value::Val<$type>> {
                &mut self.$property
            }
        }
    };
}

macro_rules! impl_unify_eq {
    ($domain:ty, $type:ty) => {
        impl<'a> crate::domain::UnifyIn<'a, OfTwo> for $type {
            fn unify_with(&self, other: &Self) -> crate::domain::Unified<'a, OfTwo> {
                if self == other {
                    crate::domain::Unified::Success
                } else {
                    crate::domain::Unified::Failed
                }
            }
        }
    };
}

macro_rules! create_types {
    ($domain:ident $($name:ident:$type:ty,)+) => {

        // #[derive(std::fmt::Debug)]
        // pub struct $domain {
        //     $($name: im::HashMap<crate::value::LVar<$type>, crate::value::Val<$type>>),+
        // }

        #[allow(non_camel_case_types)]
        #[derive(std::fmt::Debug)]
        pub enum DomainValue {
            $($name(crate::value::Val<$type>)),+
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

        impl<'a> crate::domain::Domain<'a> for $domain {
            type Value = DomainValue;
            fn new() -> Self {
                $domain {
                    $($name: im::HashMap::new(),)+
                }
            }
            fn unify_domain_values(
                state: crate::state::State<'a, Self>,
                a: Self::Value,
                b: Self::Value,
            ) -> Option<crate::state::State<Self>> {
                match (a, b) {
                    $((DomainValue::$name(a), DomainValue::$name(b)) => state.unify::<$type, crate::value::Val<$type>, crate::value::Val<$type>>(a, b),)+
                    _ => None, // This should only happen if a DomainVal constructor allows two values with different types.
                }
            }
        }

        $(impl_into_domain_val!($type, DomainValue::$name, DomainValue);)+
        $(impl_domain_type!($domain, $type, $name);)+
        $(impl_unify_eq!($domain, $type);)+

    }
}

canrun_codegen::hello! {
    OfTwo
    i32,
    Vec<i32>,
    String,
}

create_types! {
    OfTwo
    t0: i32,
    t1: Vec<i32>,
    t2: String,
}

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
