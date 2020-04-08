/// Create a `Vec<Val<T>>` without needing to manually wrap each item in [`Val`](canrun::value::Val).
///
/// The primary benefit is that it allows freely mixing resolved values and [`LVar`s](canrun::value::LVar).
///
/// # Example:
/// ```
/// use canrun::{val, var, unify, Goal, util};
/// use canrun::domains::example::I32;
/// use canrun_collections::lvec;
///
/// let x = var();
/// let hard_mode = vec![val!(x), val!(2)];
/// let easy_mode = lvec![x, 2];
///
/// assert_eq!(hard_mode, easy_mode);
/// ```
#[macro_export]
macro_rules! lvec {
    ($($item:expr),*) => {
        vec![$(canrun::value::IntoVal::into_val($item)),*]
    };
}
