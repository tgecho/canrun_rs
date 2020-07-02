use super::{LVar, Val};
use std::fmt::Debug;
use std::rc::Rc;

/// Helper for converting into [`Val<T>`](crate::value::Val).
///
/// In order to be able to mix [resolved values](crate::value::Val) and [logical
/// variables](crate::value::LVar) in the same [state](crate::state), they need
/// to be contained in the shared [`Val`](crate::value::Val) enum. This trait
/// provides a standard way to convert various types of values into this
/// container enum without manual wrapping.
///
/// # TLDR: If you see a function that takes `IntoVal<T>`
/// ```
/// # use canrun::{Val, IntoVal};
/// # use std::fmt::Debug;
/// fn foo<T: Debug, TV: IntoVal<T>>(bar: TV) -> Val<T> {
///      bar.into_val()
/// }
/// ```
/// That means it can take any of these types and will take care of converting
/// them into a `Val<T>` for you:
/// ```
/// # use canrun::{Val, var, IntoVal};
/// # use std::fmt::Debug;
/// # fn foo<T: Debug, TV: IntoVal<T>>(bar: TV) -> Val<T> {
/// #     bar.into_val()
/// # }
/// let a: Val<i32> = foo(1); // a plain value of type `T`
/// let b: Val<i32> = foo(var()); // an `LVar<T>`
/// let c: Val<i32> = foo(a); // a `Val<T>`
/// ```
pub trait IntoVal<T: Debug> {
    /// Convert various `T` related values into a [`Val<T>`](crate::value::Val).
    ///
    /// # Example:
    /// ```
    /// use canrun::{var, IntoVal, Val, LVar};
    ///
    /// let x: LVar<i32> = var();
    /// let x_val: Val<i32> = x.into_val();
    ///
    /// let y: i32 = 1;
    /// let y_val: Val<i32> = y.into_val();
    /// ```
    fn into_val(self) -> Val<T>;
}

impl<T: Debug> IntoVal<T> for T {
    fn into_val(self) -> Val<T> {
        Val::Resolved(Rc::new(self))
    }
}

impl<T: Debug> IntoVal<T> for Val<T> {
    fn into_val(self) -> Val<T> {
        self
    }
}

impl<T: Debug> IntoVal<T> for &Val<T> {
    fn into_val(self) -> Val<T> {
        self.clone()
    }
}

impl<T: Clone + Debug> IntoVal<T> for &T {
    fn into_val(self) -> Val<T> {
        Val::Resolved(Rc::new(self.clone()))
    }
}

impl<T: Debug> IntoVal<T> for LVar<T> {
    fn into_val(self) -> Val<T> {
        Val::Var(self)
    }
}
impl<T: Debug> IntoVal<T> for &LVar<T> {
    fn into_val(self) -> Val<T> {
        Val::Var(*self)
    }
}
