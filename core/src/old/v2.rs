#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unreachable_code)]
#![allow(unused_variables)]
use crate::can::lvar::LVar;
use im::HashMap;
use std::rc::Rc;

trait Unify<Other = Self> {
    fn unify_with(&self, other: &Other) -> bool;
}

impl<T: Eq> Unify for T {
    fn unify_with(&self, other: &Self) -> bool {
        self == other
    }
}

#[derive(Clone)]
enum Val<T> {
    Var(LVar),
    Res(Rc<T>),
}

use Val::{Res, Var};

fn r<T: Unify>(t: T) -> Val<T> {
    Val::Res(Rc::new(t))
}
fn lvar() -> LVar {
    LVar::new()
}

impl LVar {
    fn val<T>(&self) -> Val<T> {
        Val::Var(*self)
    }
}

#[derive(Clone)]
struct State<T: Unify> {
    pub(crate) values: HashMap<LVar, Val<T>>,
}

impl<T: Unify> State<T> {
    fn new() -> Self {
        State {
            values: HashMap::new(),
        }
    }

    fn equal(self, a: Val<T>, b: Val<T>) -> Self {
        todo!()
    }

    fn either(self, a: State<T>, b: State<T>) -> Self {
        /*
        With the base equal, we're unifying directly into the state and enjoying an implicit "both" or "all".
        We can also early detect a failure and give an easy way to check if it's known yet.
        Either adds a split. We already thought about an optimization where we push the splits to the end to
        minimize duplicate work. What if we do that here? We could start to resolve them without modifying
        the containing state, as soon as one of them fails we can ditch it and inline the other. Otherwise
        we can store both states somehow and make each merged posibility available at the end.
        */
        todo!()
    }
}

trait MemberOf<'a, T: Unify, C: Unify> {
    fn member_of(&mut self, needle: Val<T>, haystack: Val<C>) -> &mut Self;
}
impl<'a, D, T> MemberOf<'a, T, Vec<T>> for D
where
    D: Constrain2<T, Vec<T>>,
    T: Unify + Eq,
{
    fn member_of(&mut self, needle: Val<T>, haystack: Val<Vec<T>>) -> &mut Self {
        self.constrain2(needle, haystack, |n, h| match (n, h) {
            (Res(n), Res(h)) => Ok(h.contains(&n)),
            vars => Err(todo!("need to pull out the actual lvars?")),
        })
    }
}

struct Domain {
    numbers: State<i32>,
    vecs: State<Vec<i32>>,
}

struct DomainConstraints {
    numbers: Option<Vec<LVar>>,
    vecs: Option<Vec<LVar>>,
}

impl Domain {
    fn new() -> Self {
        Domain {
            numbers: State::new(),
            vecs: State::new(),
        }
    }

    fn constrain_domain<F>(&mut self, f: F) -> &mut Self
    where
        F: Fn(&Self) -> Option<DomainConstraints>,
    {
        // How does this work with the map constraints?
        match f(self) {
            Some(keys) => todo!("store the constraint based on the remaining keys?"),
            None => todo!("remove this constraint?"),
        }
    }

    // fn run(&self) -> impl Iterator<Item = ()> {
    //     // do we have some sort of proxy object with per type query fns?
    //     todo!()
    // }
}

/*
"Pending" vs "Resolved" state: We start with a Pending state and continue returning/updating as constraints are added.
Crucially, any goals that cause the state to diverge may not be fully explored.

In order to query for the current value of a variable, the state needs to be Resolved. Calling run (or one iter?) returns
an iterator of Resolved states, where each potential branch has been explored. Constraints can be added to a Resolved state,
though it will switch it back to a Pending state.

Maybe a discrete "Impossible" state might make sense. As soon as we determine that a state is unresolvable, we can basically
ignore future constraints and automatically return an empty iterator. It can support everything that other states do. I'm not
sure if this might complicate traits.
*/

trait Constrain1<A: Unify> {
    type Constraints;

    fn constrain1<F>(&mut self, a: Val<A>, f: F) -> &mut Self
    where
        F: Fn(Val<A>) -> Result<bool, Vec<LVar>>;
}

trait Constrain2<A: Unify, B: Unify> {
    type Constraints;

    fn constrain2<F>(&mut self, a: Val<A>, b: Val<B>, f: F) -> &mut Self
    where
        F: Fn(Val<A>, Val<B>) -> Result<bool, (Vec<LVar>, Vec<LVar>)>;
}

macro_rules! impl_constrain {
    ($a:ty) => {
        impl Constrain1<$a> for Domain {
            type Constraints = DomainConstraints;

            fn constrain1<F>(&mut self, a: Val<$a>, f: F) -> &mut Self
            where
                F: Fn(Val<$a>) -> Result<bool, Vec<LVar>>,
            {
                let a = todo!("self.number.resolve(a)");
                let unfulfilled = f(a).map_err(|numbers| DomainConstraints {
                    numbers: Some(numbers),
                    vecs: None,
                });
                self
            }
        }
    };

    ($a:ty, $b:ty) => {
        impl Constrain2<$a, $b> for Domain {
            type Constraints = DomainConstraints;

            fn constrain2<F>(&mut self, a: Val<$a>, b: Val<$b>, f: F) -> &mut Self
            where
                F: Fn(Val<$a>, Val<$b>) -> Result<bool, (Vec<LVar>, Vec<LVar>)>,
            {
                let a = todo!("self.number.resolve(a)");
                let b = todo!("self.number.resolve(b)");
                let unfulfilled = f(a, b).map_err(|(numbers, vecs)| DomainConstraints {
                    numbers: Some(numbers),
                    vecs: Some(vecs),
                });
                todo!("store unfulfilled somehow");
                self
            }
        }
    };
}

impl_constrain!(i32);
impl_constrain!(Vec<i32>);
impl_constrain!(i32, Vec<i32>);
impl_constrain!(Vec<i32>, i32);

fn main() {
    let (a, b, c) = (lvar().val(), lvar().val(), lvar().val());
    let numbers = State::new()
        .equal(a.clone(), r(1))
        .either(
            State::new().equal(b.clone(), r(2)),
            State::new().equal(b.clone(), r(3)),
        )
        .equal(c.clone(), r(3));

    let x = lvar().val();
    let vecs = State::new().equal(x.clone(), r(vec![1, 2, 3]));
    // .constrain(&mut numbers, a.val().member_of(x.val()));

    let mut domain = Domain::new();
    domain.constrain2(a.clone(), x.clone(), |a, x| match (a, x) {
        (Res(a), Res(x)) => {
            // ugh... I think this is a bad sign for my use of Rc<T>
            Ok((&x).iter().find(|i| **i == *a).is_some())
        }
        vars => Err(todo!("need to pull out the actual lvars?")),
    });
    // proves that we can reverse the constraint trait
    domain.constrain2(x.clone(), a.clone(), |a, x| match (a, x) {
        (Res(a), Res(x)) => Ok(a.contains(&x)),
        vars => Err(todo!("need to pull out the actual lvars?")),
    });

    // proves that we can probably abstract these constraints into a tidy trait
    domain.member_of(a.clone(), x.clone());

    // shows the pretty reasonable type error message we get for a type not in the domain
    // let w: Val<&'static str> = lvar().val();
    // domain.constrain1(w, |w| match (w) {
    //     (Res(w)) => Ok(w == "wat"),
    //     vars => Err(todo!("need to pull out the actual lvars?")),
    // });

    // this isn't going to work... what is the actual return type??!
    // let query = domain.query(a).and(x);
}
