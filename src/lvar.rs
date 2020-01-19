use std::hash::Hash;

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub struct LVar(u32);

impl LVar {
    pub fn new() -> LVar {
        LVar(rand::random())
    }
}

#[cfg(test)]
mod tests {
    use crate::lvar::LVar;
    #[test]
    fn lvar_equality() {
        let x = LVar::new();
        assert_eq!(x, x);
        assert_ne!(x, LVar::new());
    }
}
