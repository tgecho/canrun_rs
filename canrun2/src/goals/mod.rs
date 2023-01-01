/*!
Declarative relationships between values.

[`Goals`](crate::goals) provide a high level interface for defining logic
programs. They are composable, with many higher level goals being made
up of lower level primitives. Since the typical way of using goals are
through simple functions, it is easy to build and reuse custom, first class
goal constructors.

While [`State`] exposes a lower level API, in practice there really
shouldn't be anything that can't be expressed using goals.

Most structs that implement `Goal` are constructed with one of the many
[constructor functions](crate::goals#functions) and
[macros](crate::goals#macros). These high level methods provide automatic
[`Value`](crate::Value) wrapping through [`Into<Value<T>>`]
and other niceties.
*/

use std::{fmt::Debug, rc::Rc};

use crate::core::State;

mod all;
mod any;
mod both;
mod custom;
mod either;
mod fail;
mod lazy;
pub mod project;
mod succeed;
mod unify;

pub use all::*;
pub use any::*;
pub use both::*;
pub use custom::*;
pub use either::*;
pub use fail::*;
pub use lazy::*;
pub use project::*;
pub use succeed::*;
pub use unify::*;

/**
Types implementing `Goal` represent declarative, lazily applied state updates.
*/
pub trait Goal: Debug + 'static {
    /**
    Apply the `Goal` to a state, returning `Some` if the state is still valid, or `None`.

    # Example:
    ```
    use canrun2::{State, Query, Value};
    use canrun2::goals::Goal;

    #[derive(Debug)]
    struct Is1 {value: Value<usize>}

    impl Goal for Is1 {
        fn apply(&self, state: State) -> Option<State> {
            state.unify(&Value::new(1), &self.value)
        }
    }

    let x = Value::var();
    let goal = Is1 {value: x.clone()};
    let results: Vec<_> = goal.query(x).collect();
    assert_eq!(results, vec![1]);
    ```
    */
    fn apply(&self, state: State) -> Option<State>;
}

impl Goal for Rc<dyn Goal> {
    fn apply(&self, state: State) -> Option<State> {
        self.as_ref().apply(state)
    }
}
