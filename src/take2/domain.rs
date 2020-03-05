use super::val::Val;
use crate::LVar;
use im::HashMap;

pub trait Domain {
    fn new() -> Self;
}

pub trait DomainType<T>: Domain {
    fn values_as_ref(&self) -> &HashMap<LVar, Val<T>>;
    fn values_as_mut(&mut self) -> &mut HashMap<LVar, Val<T>>;
}

#[derive(Clone)]
pub struct Just<T> {
    values: HashMap<LVar, Val<T>>,
}

impl<'a, T> Domain for Just<T> {
    fn new() -> Self {
        Just {
            values: HashMap::new(),
        }
    }
}

impl<'a, T> DomainType<T> for Just<T> {
    fn values_as_ref(&self) -> &HashMap<LVar, Val<T>> {
        &self.values
    }
    fn values_as_mut(&mut self) -> &mut HashMap<LVar, Val<T>> {
        &mut self.values
    }
}
