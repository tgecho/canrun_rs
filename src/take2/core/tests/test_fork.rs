use super::super::domain::{Domain, Just};
use super::super::state::{State, StateIter};
use super::super::val::{val, var, Val};
use std::iter::repeat;
use std::rc::Rc;

fn any<'a, D>(
    funcs: Vec<Rc<dyn Fn(State<'a, D>) -> Result<State<'a, D>, State<'a, D>>>>,
) -> Rc<dyn Fn(State<'a, D>) -> StateIter<'a, State<'a, D>> + 'a>
where
    D: Domain + 'a,
{
    Rc::new(move |s: State<D>| {
        Box::new(
            funcs
                .clone()
                .into_iter()
                .zip(repeat(s))
                .filter_map(|(f, s)| f(s).ok()),
        )
    })
}

#[test]
fn basic_fork() {
    let s: State<Just<i32>> = State::new();
    let s = s.fork(Rc::new(|s| Box::new(s.unify(val(2), val(2)).into_iter())));
    assert_eq!(1, s.unwrap().iter().count());
}

#[test]
fn basic_fork_either() {
    let s: State<Just<i32>> = State::new();
    let first = s.fork(any(vec![
        Rc::new(|s: State<Just<i32>>| s.unify(val(2), val(2))),
        Rc::new(|s: State<Just<i32>>| s.unify(val(1), val(2))),
    ]));
    assert_eq!(1, first.unwrap().iter().count());
}
