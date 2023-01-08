use crate::core::Unify;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{any::Any, fmt::Debug, rc::Rc};

pub(crate) type VarId = usize;

/** A logical variable that represents a potential value of type `T`.

`LVars` are are passed into [goals](crate::goals) to relate
[values](crate::Value) and other variables to each other. They can also be
used to [query](crate::Query) for values.

The main reason to deal with an `LVar` directly as opposed to just
[`Value::Var`](crate::core::Value::Var) is that `LVar` implements [`Copy`],
which makes it more convenient when the same var is shared between various goals,
closures and other places.

The identity of each `LVar` is tracked using an internal id. While this id
is visible through the `Debug` implementation, it should only be used for
debugging purposes as no guarantees are made about the type or generation of
the id value. Also, these ids are only valid within the context of a single
execution. They cannot be safely persisted or shared between processes.
*/
#[derive(Debug, Copy, Eq)]
pub struct LVar<T> {
    pub(crate) id: VarId,
    t: PhantomData<T>,
}

impl<T> PartialEq for LVar<T> {
    fn eq(&self, other: &LVar<T>) -> bool {
        self.id == other.id
    }
}

impl<T> Hash for LVar<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

fn get_id() -> VarId {
    static COUNTER: AtomicUsize = AtomicUsize::new(1);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}
impl<T: Unify> LVar<T> {
    /// Create a new [logical var](LVar).
    ///
    /// # Example:
    /// ```
    /// use canrun::LVar;
    /// let x: LVar<isize> = LVar::new();
    /// ```
    pub fn new() -> Self {
        LVar {
            id: get_id(),
            t: PhantomData::default(),
        }
    }
}

impl<T: Unify> Default for LVar<T> {
    fn default() -> Self {
        Self::new()
    }
}

/**
Contain individual resolved values or variables that can be bound through
[unification](crate::core::Unify).
*/
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Value<T: Unify> {
    /**
    A [logical variable](LVar).

    Even though they do not contain a value, they are parameterized with the type
    they can potentially be bound to. This ensures that they can only be unified
    with values of the same type.
    */
    Var(LVar<T>),
    /** A resolved value.

    When a state is split into an arbitrary number of [resolved
    states](crate::core::State), some of the internal data
    structures often need to be cloned. In an attempt to avoid unnecessary
    cloning of every value in the state, we wrap it in an [Rc] so that
    references can be shared.
    */
    Resolved(Rc<T>),
}

impl<T: Unify> Value<T> {
    /// Create a new [logical `Value`](Value) wrapping a `T`.
    ///
    /// # Example:
    /// ```
    /// use canrun::Value;
    /// let x: Value<i32> = Value::new(1);
    /// ```
    pub fn new(t: T) -> Value<T> {
        Value::Resolved(Rc::new(t))
    }
    /// Create a new [logical `Value`](Value) with an unresolved [`Var`](Value::Var).
    ///
    /// This is a shorthand for `Value::Var(LVar::new())`.
    ///
    /// # Example:
    /// ```
    /// use canrun::Value;
    /// let x: Value<i32> = Value::var();
    /// ```
    pub fn var() -> Value<T> {
        Value::Var(LVar::new())
    }

    pub(crate) fn to_anyval(&self) -> AnyVal {
        match self {
            Value::Var(var) => AnyVal::Var(var.id),
            Value::Resolved(val) => AnyVal::Resolved(val.clone()),
        }
    }

    /** Return `T` if the `Value` is a resolved.

    Example:
    ```
    use canrun::Value;

    let x: Value<i32> = Value::new(1);
    assert_eq!(x.resolved(), Some(&1));
    ```
    */
    pub fn resolved(&self) -> Option<&T> {
        match self {
            Value::Var(_) => None,
            Value::Resolved(val) => Some(val.as_ref()),
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) enum AnyVal {
    Var(VarId),
    Resolved(Rc<dyn Any>),
}

impl AnyVal {
    pub fn to_value<T: Unify>(&self) -> Option<Value<T>> {
        match self {
            AnyVal::Var(id) => Some(Value::Var(LVar {
                id: *id,
                t: PhantomData::default(),
            })),
            AnyVal::Resolved(val) => {
                let rc_t = val.clone().downcast::<T>().ok()?;
                Some(Value::Resolved(rc_t))
            }
        }
    }

    pub fn is_resolved(&self) -> bool {
        matches!(self, AnyVal::Resolved(_))
    }
}

impl<T: Unify> From<LVar<T>> for Value<T> {
    fn from(var: LVar<T>) -> Self {
        Value::Var(var)
    }
}

impl<T: Unify> From<&LVar<T>> for Value<T> {
    fn from(var: &LVar<T>) -> Self {
        Value::Var(var.clone())
    }
}

impl<T: Unify> From<&Value<T>> for Value<T> {
    fn from(var: &Value<T>) -> Self {
        var.clone()
    }
}

impl<T: Unify> From<T> for Value<T> {
    fn from(t: T) -> Self {
        Value::Resolved(Rc::new(t))
    }
}

// These manual Clone impls are needed because the derive macro adds a `T:
// Clone` constraint. See
// https://doc.rust-lang.org/std/clone/trait.Clone.html#derivable and
// https://stegosaurusdormant.com/understanding-derive-clone/

impl<T> Clone for LVar<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            t: self.t,
        }
    }
}

impl<T: Unify> Clone for Value<T> {
    fn clone(&self) -> Self {
        match self {
            Self::Var(v) => Self::Var(v.clone()),
            Self::Resolved(v) => Self::Resolved(v.clone()),
        }
    }
}
