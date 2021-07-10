use std::fmt;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::sync::atomic::{AtomicUsize, Ordering};

pub(in super::super) type LVarId = usize;

fn get_id() -> LVarId {
    static COUNTER: AtomicUsize = AtomicUsize::new(1);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

/** A logical variable that represents a potential value of type `T`.

They are typically created with the [`var()`](crate::value::var) function.

`LVars` are are passed into [goals](crate::goals) to relate
[values](crate::value) and other variables to each other. They can also be
used to [query](crate::Query) for values in a
[`ResolvedState`](crate::state::ResolvedState).

The identity of each `LVar` is tracked using an internal id. While this id
is visible through the `Debug` implementation, it should only be used for
debugging purposes as no guarantees are made about the type or generation of
the id value.
*/
#[derive(Default)]
pub struct LVar<T: ?Sized> {
    pub(in super::super) id: LVarId,
    label: Option<&'static str>,
    t: PhantomData<T>,
}

/// Create a new [logical var](LVar).
///
/// This is simply a shorthand for [`LVar::new()`].
///
/// # Example:
/// ```
/// use canrun::{var, LVar};
///
/// let x: LVar<i32> = var();
/// ```
pub fn var<T>() -> LVar<T> {
    LVar::new()
}

impl<T> PartialEq for LVar<T> {
    fn eq(&self, other: &LVar<T>) -> bool {
        self.id == other.id
    }
}
impl<T> Eq for LVar<T> {}

impl<T> LVar<T> {
    /// Create a new [logical var](LVar).
    ///
    /// The [`var()`](crate::value::var) function is typically used as a
    /// shorthand.
    ///
    /// # Example:
    /// ```
    /// use canrun::{LVar};
    ///
    /// let x: LVar<i32> = LVar::new();
    /// ```
    pub fn new() -> LVar<T> {
        LVar {
            id: get_id(),
            label: None,
            t: PhantomData,
        }
    }

    /// Create a labeled [logical var](LVar).
    ///
    /// `LVars` are primarily represented by an internal id. A textual label can
    /// assist in debugging.
    ///
    /// No guarantees are made about the actual debug string. Two `LVars`
    /// created separately are not considered to be the same, even if they
    /// have the same label.
    ///
    /// # Examples:
    /// ```
    /// use canrun::{LVar};
    ///
    /// let x: LVar<i32> = LVar::labeled("foo");
    /// assert!(format!("{:?}", x).contains("foo"));
    /// ```
    /// ```
    /// # use canrun::{LVar};
    /// let x: LVar<i32> = LVar::labeled("foo");
    /// let y: LVar<i32> = LVar::labeled("foo");
    /// assert_eq!(x, x);
    /// assert_ne!(x, y);
    /// ```
    pub fn labeled(label: &'static str) -> LVar<T> {
        LVar {
            id: get_id(),
            label: Some(label),
            t: PhantomData,
        }
    }
}

impl<T> Hash for LVar<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<T> fmt::Debug for LVar<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.label {
            Some(label) => write!(f, "LVar({}/{})", self.id, label),
            None => write!(f, "LVar({})", self.id),
        }
    }
}

impl<T> Clone for LVar<T> {
    fn clone(&self) -> Self {
        LVar {
            id: self.id,
            label: self.label,
            t: self.t,
        }
    }
}
impl<T> Copy for LVar<T> {}

#[cfg(test)]
mod tests {
    use super::LVar;

    #[test]
    fn lvar_equality() {
        let x: LVar<()> = LVar::new();
        assert_eq!(x, x);
        assert_ne!(x, LVar::new());
    }
    #[test]
    fn lvar_labels() {
        let a: LVar<()> = LVar::labeled("a");
        // Matching labels do not make them equal
        assert_ne!(a, LVar::labeled("a"));
        // Mismatched labels do not negate matching ids
        // (though you shouldn't try to do this)
        assert_eq!(
            a,
            LVar {
                label: Some("b"),
                ..a
            }
        );
    }
}
