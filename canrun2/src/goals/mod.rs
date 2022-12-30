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

pub mod all;
pub mod any;
pub mod both;
pub mod either;
pub mod fail;
pub mod lazy;
pub mod project;
pub mod succeed;
pub mod unify;

pub trait Goal: Debug + 'static {
    fn apply(&self, state: State) -> Option<State>;
}

impl Goal for Rc<dyn Goal> {
    fn apply(&self, state: State) -> Option<State> {
        self.as_ref().apply(state)
    }
}
