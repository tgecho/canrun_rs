//! Comparison goals including [`lt`](lt::lt), [`lte`](lte::lte),
//! [`gt`](gt::gt), [`gte`](gte::gte), [`min`](min::min) and [`max`](max::max).

mod gt;
mod gte;
mod lt;
mod lte;
mod max;
mod min;

pub use gt::gt;
pub use gte::gte;
pub use lt::lt;
pub use lte::lte;
pub use max::max;
pub use min::min;
