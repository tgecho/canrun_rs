//! Run code when [`variables`](crate::value::LVar) are resolved.

use crate::value::{
    LVar, LVarId, Val,
    Val::{Resolved, Var},
};
use crate::{Domain, DomainType, State};
use std::fmt::Debug;
use std::rc::Rc;

/// An alias for the function that should be returned by a successful
/// [`Constraint::attempt`] to update the [`State`].
pub type ResolveFn<'a, D> = Box<dyn FnOnce(State<'a, D>) -> Option<State<'a, D>> + 'a>;

/// Update a [`State`] whenever one or more [`LVar`]s are resolved.
///
/// The [`Constraint::attempt`] function will be run when it is initially added.
/// Returning a `Err([`VarWatch`])` signals that the constraint is not
/// satisfied. It will be re-run when one of the specified variables is bound to
/// another value.
///
/// You probably want the higher level [goal projection](crate::goal::project)
/// functions.
///
/// # NOTE:
/// The [`attempt`](Constraint::attempt) function must take care to [fully
/// resolve](State::resolve_val) any variables before requesting that they be
/// watched. The [`resolve_1`], [`resolve_2`], [`OneOfTwo`] and [`TwoOfThree`]
/// helpers can simplify handling this (plus returning a [`VarWatch`]).
///
/// # Example:
/// ```
/// use canrun::{State, Query, Val, val, var, DomainType};
/// use canrun::state::constraints::{Constraint, resolve_1, ResolveFn, VarWatch};
/// use canrun::example::I32;
/// use std::rc::Rc;
/// use std::fmt;
///
/// struct Assert<'a, T: fmt::Debug> {
///     val: Val<T>,
///     assert: Rc<dyn Fn(&T) -> bool + 'a>,
/// }
///
/// impl<'a, T: fmt::Debug> fmt::Debug for Assert<'a, T> {
///     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
///         write!(f, "Assert({:?})", self.val)
///     }
/// }
///
/// impl<'a, T, D> Constraint<'a, D> for Assert<'a, T>
/// where
///     T: fmt::Debug + 'a,
///     D: DomainType<'a, T>,
/// {
///     fn attempt(&self, state: &State<'a, D>) -> Result<ResolveFn<'a, D>, VarWatch> {
///         let resolved = resolve_1(&self.val, state)?;
///         let assert = self.assert.clone();
///         Ok(Box::new(
///             move |state: State<'a, D>| if assert(&*resolved) { Some(state) } else { None },
///         ))
///     }
/// }
///
/// # fn test() -> Option<()> {
/// let x = var();
///
/// let state: State<I32> = State::new();
/// let state = state.constrain(Rc::new(Assert {val: val!(x), assert: Rc::new(|x| x > &1)}));
/// let state = state?.unify(&val!(x), &val!(2));
///
/// let results: Vec<i32> = state.query(x).collect();
/// assert_eq!(results, vec![2]);
/// # Some(())
/// # }
/// # test();
/// ```
pub trait Constraint<'a, D>: Debug
where
    D: Domain<'a>,
{
    /// Resolve required variables in a state and resubscribe or request to
    /// update the state.
    fn attempt(&self, state: &State<'a, D>) -> Result<ResolveFn<'a, D>, VarWatch>;
}

/// A set of variables to watch on behalf of a [constraint
/// object](crate::state::State::constrain()).
///
/// Consider generating this with the [`resolve_1`], [`resolve_2`], [`OneOfTwo`]
/// or [`TwoOfThree`] helpers.
#[derive(Debug)]
pub struct VarWatch(pub(crate) Vec<LVarId>);

impl VarWatch {
    /// Watch one [`LVar`] for changes in a [`Constraint`].
    pub fn one<A>(a: LVar<A>) -> Self {
        VarWatch(vec![a.id])
    }

    /// Watch two [`LVar`]s for changes in a [`Constraint`].
    pub fn two<A, B>(a: LVar<A>, b: LVar<B>) -> Self {
        VarWatch(vec![a.id, b.id])
    }
}

/// Resolve one [`Val`] or return an [`Err(VarWatch)`](VarWatch) in a
/// [`Constraint`].
pub fn resolve_1<'a, A, D>(val: &Val<A>, state: &State<'a, D>) -> Result<Rc<A>, VarWatch>
where
    A: Debug,
    D: DomainType<'a, A>,
{
    let a = state.resolve_val(val);
    match a {
        Resolved(a) => Ok(a.clone()),
        Var(var) => Err(VarWatch::one(*var)),
    }
}

/// Resolve two [`Val`]s or return an [`Err(VarWatch)`](VarWatch) in a
/// [`Constraint`].
pub fn resolve_2<'a, A, B, D>(
    a: &Val<A>,
    b: &Val<B>,
    state: &State<'a, D>,
) -> Result<(Rc<A>, Rc<B>), VarWatch>
where
    A: Debug,
    B: Debug,
    D: DomainType<'a, A> + DomainType<'a, B>,
{
    let a = state.resolve_val(a);
    let b = state.resolve_val(b);
    match (a, b) {
        (Resolved(a), Resolved(b)) => Ok((a.clone(), b.clone())),
        (Var(var), _) => Err(VarWatch::one(*var)),
        (_, Var(var)) => Err(VarWatch::one(*var)),
    }
}

/// Resolve one out of two [`Val`]s or return an [`Err(VarWatch)`](VarWatch) in
/// a [`Constraint`].
pub enum OneOfTwo<A: Debug, B: Debug> {
    /// Returned when the first [`Val`] is successfully resolved.
    A(Rc<A>, Val<B>),
    /// Returned when the second [`Val`] is successfully resolved.
    B(Val<A>, Rc<B>),
}

impl<A: Debug, B: Debug> OneOfTwo<A, B> {
    /// Attempt to resolve a [`OneOfTwo`] enum from a [`State`].
    pub fn resolve<'a, D>(
        a: &Val<A>,
        b: &Val<B>,
        state: &State<'a, D>,
    ) -> Result<OneOfTwo<A, B>, VarWatch>
    where
        D: DomainType<'a, A> + DomainType<'a, B>,
    {
        let a = state.resolve_val(a);
        let b = state.resolve_val(b);
        match (a, b) {
            (Resolved(a), b) => Ok(OneOfTwo::A(a.clone(), b.clone())),
            (a, Resolved(b)) => Ok(OneOfTwo::B(a.clone(), b.clone())),
            (Var(a), Var(b)) => Err(VarWatch::two(*a, *b)),
        }
    }
}

/// Resolve two out of three [`Val`]s or return an [`Err(VarWatch)`](VarWatch)
/// in a [`Constraint`].
pub enum TwoOfThree<A: Debug, B: Debug, C: Debug> {
    /// Returned when the first and second [`Val`]s are successfully resolved.
    AB(Rc<A>, Rc<B>, Val<C>),
    /// Returned when the second and third [`Val`]s are successfully resolved.
    BC(Val<A>, Rc<B>, Rc<C>),
    /// Returned when the first and third [`Val`]s are successfully resolved.
    AC(Rc<A>, Val<B>, Rc<C>),
}

impl<A: Debug, B: Debug, C: Debug> TwoOfThree<A, B, C> {
    /// Attempt to resolve a [`TwoOfThree`] enum from a [`State`].
    pub fn resolve<'a, D>(
        a: &Val<A>,
        b: &Val<B>,
        c: &Val<C>,
        state: &State<'a, D>,
    ) -> Result<TwoOfThree<A, B, C>, VarWatch>
    where
        D: DomainType<'a, A> + DomainType<'a, B> + DomainType<'a, C>,
    {
        let a = state.resolve_val(a);
        let b = state.resolve_val(b);
        let c = state.resolve_val(c);
        match (a, b, c) {
            (Resolved(a), Resolved(b), c) => Ok(TwoOfThree::AB(a.clone(), b.clone(), c.clone())),
            (a, Resolved(b), Resolved(c)) => Ok(TwoOfThree::BC(a.clone(), b.clone(), c.clone())),
            (Resolved(a), b, Resolved(c)) => Ok(TwoOfThree::AC(a.clone(), b.clone(), c.clone())),
            (Var(a), Var(b), _) => Err(VarWatch::two(*a, *b)),
            (Var(a), _, Var(c)) => Err(VarWatch::two(*a, *c)),
            (_, Var(b), Var(c)) => Err(VarWatch::two(*b, *c)),
        }
    }
}
