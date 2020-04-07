use super::Val;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::sync::atomic::{AtomicUsize, Ordering};

pub(in super::super) type LVarId = usize;

fn get_id() -> LVarId {
    static COUNTER: AtomicUsize = AtomicUsize::new(1);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

#[derive(Default)]
pub struct LVar<T: ?Sized> {
    pub(in super::super) id: LVarId,
    label: Option<&'static str>,
    t: PhantomData<T>,
}

/// Create a new [logical var](LVar).
///
/// This is simply a shorthand for [LVar::new()].
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
    pub fn new() -> LVar<T> {
        LVar {
            id: get_id(),
            label: None,
            t: PhantomData,
        }
    }
    pub fn labeled(label: &'static str) -> LVar<T> {
        LVar {
            id: get_id(),
            label: Some(label),
            t: PhantomData,
        }
    }
    pub fn into_val(self) -> Val<T> {
        Val::Var(self)
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
