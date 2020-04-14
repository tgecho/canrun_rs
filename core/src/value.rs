//! Contain individual resolved values or variables that can be bound through
//! [unification](crate::UnifyIn).
//!
//! Values are parameterized with the type they can contain. This ensures that
//! they can only be unified with values of the same type, and they can only be
//! added to [states](crate::state) with a compatible [domain](crate::domains).
mod into_val;
mod lvar;
mod reify_in;

pub use into_val::IntoVal;
pub(super) use lvar::LVarId;
pub use lvar::{var, LVar};
pub use reify_in::ReifyIn;
use std::fmt;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

/// The possible states a value can be in.
pub enum Val<T: Debug + ?Sized> {
    /// A [logical variable](LVar).
    Var(LVar<T>),
    /// A resolved value.
    ///
    /// When a state is split into an arbitrary number of [resolved
    /// states](crate::state::ResolvedState), some of the internal data
    /// structures often need to be cloned. In an attempt to avoid unnecessary
    /// cloning of every value in the state, we wrap it in an [Rc] so that
    /// references can be shared.
    Resolved(Rc<T>),
}

use Val::{Resolved, Var};

impl<T: Debug> Val<T> {
    /// Attempt to extract a reference to resolved value (`&T`) or return the
    /// `LVar` if the value is not yet resolved.
    ///
    /// Examples:
    /// ```
    /// use canrun::{var, val, LVar};
    ///
    /// let x: LVar<i32> = var();
    /// let x_val = val!(x);
    /// assert_eq!(x_val.resolved(), Err(x));
    /// ```
    /// ```
    /// # use canrun::{var, val};
    /// let y_val = val!(1);
    /// assert_eq!(y_val.resolved(), Ok(&1));
    /// ```
    pub fn resolved(&self) -> Result<&T, LVar<T>> {
        match self {
            Resolved(x) => Ok(&*x),
            Var(x) => Err(*x),
        }
    }

    /// Return `true` if the `Val` is an unresolved variable.
    ///
    /// Example:
    /// ```
    /// use canrun::{var, val, Val};
    ///
    /// let x: Val<i32> = val!(var());
    /// assert!(x.is_var());
    /// ```
    pub fn is_var(&self) -> bool {
        matches!(self, Var(_))
    }

    /// Return `true` if the `Val` is a resolved value.
    ///
    /// Example:
    /// ```
    /// use canrun::{var, val, Val};
    ///
    /// let x: Val<i32> = val!(1);
    /// assert!(x.is_resolved());
    /// ```
    pub fn is_resolved(&self) -> bool {
        matches!(self, Resolved(_))
    }
}

/// Easy conversion of [`LVar<T>`](LVar) and `T` values into [`Val<T>`](Val).
///
/// This simply wraps [`IntoVal`](crate::value::IntoVal) to be slightly more
/// convenient. Note that goal constructors typically do this conversion
/// automatically.
///
/// Example:
/// ```
/// use canrun::{val, var, Val, LVar};
///
/// let x: LVar<i32> = var();
/// let x_val: Val<i32> = val!(x);
///
/// let y: i32 = 1;
/// let y_val: Val<i32> = val!(y);
/// ```
#[macro_export]
macro_rules! val {
    ($value:expr) => {
        $crate::value::IntoVal::into_val($value)
    };
}

#[doc(inline)]
pub use val;

impl<T: Debug> Clone for Val<T> {
    fn clone(&self) -> Self {
        match self {
            Val::Var(var) => Val::Var(*var),
            Val::Resolved(r) => Val::Resolved(r.clone()),
        }
    }
}

impl<T: PartialEq + Debug> PartialEq for Val<T> {
    fn eq(&self, other: &Val<T>) -> bool {
        match (self, other) {
            (Resolved(s), Resolved(other)) => s == other,
            (Var(s), Var(other)) => s == other,
            _ => false,
        }
    }
}
impl<T: Eq + Debug> Eq for Val<T> {}

impl<T: Hash + Debug> Hash for Val<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Resolved(val) => val.hash(state),
            Var(var) => var.hash(state),
        }
    }
}

impl<'a, T: fmt::Debug> fmt::Debug for Val<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Resolved(v) => write!(f, "Resolved({:?})", v),
            Var(v) => write!(f, "Var({:?})", v),
        }
    }
}
