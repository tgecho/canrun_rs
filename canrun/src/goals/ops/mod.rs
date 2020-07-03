//! Operator goals including [`add`](add::add), [`sub`](sub::sub),
//! [`mul`](mul::mul) and [`div`](div::div).

mod add;
mod div;
mod mul;
mod sub;

pub use add::add;
pub use div::div;
pub use mul::mul;
pub use sub::sub;
