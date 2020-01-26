use std::fmt;
use std::hash::Hash;
use std::sync::atomic::{AtomicUsize, Ordering};

fn get_id() -> usize {
    static COUNTER: AtomicUsize = AtomicUsize::new(1);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

#[derive(Eq, Clone, Copy, Hash)]
pub struct LVar {
    id: usize,
    label: Option<&'static str>,
}

impl PartialEq for LVar {
    fn eq(&self, other: &LVar) -> bool {
        self.id == other.id
    }
}

impl LVar {
    pub fn new() -> LVar {
        LVar {
            id: get_id(),
            label: None,
        }
    }
    pub fn labeled(label: &'static str) -> LVar {
        LVar {
            id: get_id(),
            label: Some(label),
        }
    }
}

impl fmt::Debug for LVar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.label {
            Some(label) => write!(f, "LVar({}/{})", self.id, label),
            None => write!(f, "LVar({})", self.id),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cell::lvar::LVar;
    #[test]
    fn lvar_equality() {
        let x = LVar::new();
        assert_eq!(x, x);
        assert_ne!(x, LVar::new());
    }
    #[test]
    fn lvar_labels() {
        let a = LVar::labeled("a");
        // Matching labels do not make them equal
        assert_ne!(a, LVar::labeled("a"));
        // Mismatched labels do not negate matching ids
        // (though you shouldn't do this)
        assert_eq!(
            a,
            LVar {
                id: a.id,
                label: Some("b")
            }
        );
    }
}
