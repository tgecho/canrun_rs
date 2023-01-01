/*!
Declarative relationships between values.

[`Goals`](crate::goals) provide a high level interface for defining logic
programs. They are composable, with many higher level goals being made
up of lower level primitives. Since the typical way of using goals are
through simple functions, it is easy to build and reuse custom, first class
goal constructors.

While [`State`] exposes a lower level API, in practice there really
shouldn't be anything that can't be expressed using goals.
*/

use std::{fmt::Debug, rc::Rc};

use crate::core::State;

mod all;
mod any;
mod both;
mod either;
mod fail;
mod lazy;
pub mod project;
mod succeed;
mod unify;

pub use all::*;
pub use any::*;
pub use both::*;
pub use either::*;
pub use fail::*;
pub use lazy::*;
pub use project::*;
pub use succeed::*;
pub use unify::*;

pub trait Goal: Debug + 'static {
    fn apply(&self, state: State) -> Option<State>;
}

impl Goal for Rc<dyn Goal> {
    fn apply(&self, state: State) -> Option<State> {
        self.as_ref().apply(state)
    }
}
