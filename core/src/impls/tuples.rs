use crate::domains::DomainType;
use crate::state::State;
use crate::value::{ReifyIn, Val};
use crate::ResolvedState;
use crate::UnifyIn;
use std::rc::Rc;

macro_rules! impl_for_tuple {
    ($($t:ident => $r:ident),+) => {
        impl<'a, $($t,)* D> UnifyIn<'a, D> for ($(Val<$t>),*)
        where
            $($t: UnifyIn<'a, D>, )*
            D: $(DomainType<'a, $t> +)* DomainType<'a, Self>
        {
            fn unify_resolved(
                state: State<'a, D>,
                l: Rc<Self>,
                r: Rc<Self>,
            ) -> Option<State<'a, D>> {
                #![allow(non_snake_case)]
                let ($($t),*) = &*l;
                // Abusing the "reified" ident as "right" since
                // it's available. If we did this as a proc-macro
                // we could actually make up our own names.
                let ($($r),*) = &*r;
                Some(
                    state
                        $(.unify(&$t.clone(), &$r.clone())?)*
                )
            }
        }

        impl<'a, D: 'a, $($t: ReifyIn<'a, D, Reified = $r>, $r,)*> ReifyIn<'a, D> for ($($t),*) {
            type Reified = ($($t::Reified),*);
            fn reify_in(&self, state: &ResolvedState<D>) -> Option<Self::Reified> {
                #![allow(non_snake_case)]
                let ($($t),*) = self;
                Some(($($t.reify_in(state)?),*))
            }
        }
    };
}

impl_for_tuple!(Av => Ar, Bv => Br);
impl_for_tuple!(Av => Ar, Bv => Br, Cv => Cr);
impl_for_tuple!(Av => Ar, Bv => Br, Cv => Cr, Dv => Dr);
impl_for_tuple!(Av => Ar, Bv => Br, Cv => Cr, Dv => Dr, Ev => Er);

/// Create a tuple of [logical values](value::Val) with automatic [`IntoVal`
/// wrapping](value::IntoVal).
///
/// The primary benefit is that it allows freely mixing resolved values and
/// [`LVar`s](value::LVar).
///
/// # Example:
/// ```
/// use canrun::{var, ltup, Val};
/// let x = var();
/// let tuple: (Val<i32>, Val<i32>, Val<&'static str>) = ltup!(x, 1, "two");
/// ```
#[macro_export]
macro_rules! ltup {
    ($($item:expr),* $(,)?) => {
        ($($crate::value::IntoVal::into_val($item)),*)
    };
}

#[doc(inline)]
pub use ltup;

#[cfg(test)]
mod tests {
    use crate as canrun;
    use crate::goal::unify;
    use crate::goal::Goal;
    use crate::util;
    use crate::value::{var, Val};
    use canrun_codegen::domain;

    domain! {
        pub Tuples2 {
            i32,
            (Val<i32>, Val<i32>),
        }
    }
    domain! {
        pub Tuples3 {
            i32,
            (Val<i32>, Val<i32>, Val<i32>),
        }
    }

    #[test]
    fn tuple2_succeeds() {
        let x = var();
        let goals: Vec<Goal<Tuples2>> = vec![unify(x, ltup!(1, 2)), unify(x, ltup!(1, 2))];
        util::assert_permutations_resolve_to(goals, x, vec![(1, 2)]);
    }

    #[test]
    fn tuple2_fails() {
        let x = var();
        let goals: Vec<Goal<Tuples2>> = vec![unify(x, ltup!(1, 3)), unify(x, ltup!(1, 2))];
        util::assert_permutations_resolve_to(goals, x, vec![]);
    }

    #[test]
    fn tuple2_nested_var() {
        let x = var();
        let y = var();
        let goals: Vec<Goal<Tuples2>> = vec![unify(x, ltup!(1, y)), unify(x, ltup!(1, 2))];
        util::assert_permutations_resolve_to(goals, y, vec![2]);
    }

    #[test]
    fn tuple3_succeeds() {
        let x = var();
        let goals: Vec<Goal<Tuples3>> = vec![unify(x, ltup!(1, 2, 3)), unify(x, ltup!(1, 2, 3))];
        util::assert_permutations_resolve_to(goals, x, vec![(1, 2, 3)]);
    }

    #[test]
    fn tuple3_fails() {
        let x = var();
        let goals: Vec<Goal<Tuples3>> = vec![unify(x, ltup!(1, 2, 3)), unify(x, ltup!(1, 2, 4))];
        util::assert_permutations_resolve_to(goals, x, vec![]);
    }

    #[test]
    fn tuple3_nested_var() {
        let x = var();
        let y = var();
        let goals: Vec<Goal<Tuples3>> = vec![unify(x, ltup!(1, y, 3)), unify(x, ltup!(1, 2, 3))];
        util::assert_permutations_resolve_to(goals, y, vec![2]);
    }
}
