use std::sync::atomic::{AtomicUsize, Ordering};
use std::{any::Any, fmt::Debug, rc::Rc};

type LVarId = usize;

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct LVar(LVarId);

fn get_id() -> LVarId {
    static COUNTER: AtomicUsize = AtomicUsize::new(1);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

#[derive(Clone, Debug, PartialEq)]
pub enum Val<T> {
    Var(LVar),
    Value(Rc<T>),
}

#[derive(Clone, Debug)]
enum AnyVal {
    Var(LVar),
    Value(Rc<dyn Any>),
}

pub fn var<T: Unify>() -> Val<T> {
    Val::Var(LVar(get_id()))
}

pub fn val<T: Unify>(t: T) -> Val<T> {
    Val::Value(Rc::new(t))
}

impl<T: Unify> From<&Val<T>> for AnyVal {
    fn from(value: &Val<T>) -> Self {
        match value {
            Val::Var(var) => AnyVal::Var(*var),
            Val::Value(val) => AnyVal::Value(val.clone()),
        }
    }
}

impl<T: Unify> From<Val<T>> for AnyVal {
    fn from(value: Val<T>) -> Self {
        match value {
            Val::Var(var) => AnyVal::Var(var),
            Val::Value(val) => AnyVal::Value(val),
        }
    }
}

pub trait Unify: Any + Debug {
    fn unify(state: State, a: Rc<Self>, b: Rc<Self>) -> Option<State>;
}

impl Unify for usize {
    fn unify(state: State, a: Rc<Self>, b: Rc<Self>) -> Option<State> {
        if a == b {
            Some(state)
        } else {
            None
        }
    }
}

#[derive(Clone)]
pub struct State {
    values: im_rc::HashMap<LVarId, AnyVal>,
}

impl State {
    pub fn new() -> Self {
        State {
            values: im_rc::HashMap::new(),
        }
    }

    fn resolve_any<'a>(&'a self, val: &'a AnyVal) -> &'a AnyVal {
        match val {
            AnyVal::Var(var) => {
                let resolved = self.values.get(&var.0);
                match resolved {
                    Some(AnyVal::Var(found_var)) if found_var == var => val,
                    Some(found) => self.resolve_any(found),
                    None => val,
                }
            }
            value => value,
        }
    }

    pub fn resolve<T: Unify>(&self, val: &Val<T>) -> Option<Val<T>> {
        match self.resolve_any(&AnyVal::from(val)) {
            AnyVal::Var(var) => Some(Val::Var(*var)),
            AnyVal::Value(val) => {
                let rc_t = val.clone().downcast::<T>().ok()?;
                Some(Val::Value(rc_t))
            }
        }
    }

    pub fn unify<T: Unify>(mut self, a: Val<T>, b: Val<T>) -> Option<Self> {
        let a = self.resolve(&a)?;
        let b = self.resolve(&b)?;

        match (a, b) {
            (Val::Value(a), Val::Value(b)) => Unify::unify(self, a, b),
            (Val::Var(a), Val::Var(b)) if a == b => Some(self),
            (Val::Var(var), val) | (val, Val::Var(var)) => {
                // TODO: Add occurs check?
                self.values.insert(var.0, AnyVal::from(val));
                Some(self)
            }
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn unify() {
        let x = var();
        let state = State::new();

        let state = state.unify(x.clone(), val(1)).unwrap();
        assert_eq!(state.resolve::<usize>(&x).unwrap(), val(1));
    }
}
