use super::goals::Goal;
use super::val::Val;
pub trait Domain {
    fn new() -> Self;
}

pub trait DomainType<T>: Domain {
    fn values_as_ref(&self) -> &HashMap<LVar, Val<T>>;
    fn values_as_mut(&mut self) -> &mut HashMap<LVar, Val<T>>;
}

use crate::LVar;
use im::HashMap;

// #[derive(Clone)]
pub struct Just<'a, T> {
    values: HashMap<LVar, Val<T>>,
    goals: Vec<Box<dyn Goal<'a, T> + 'a>>,
}

impl<T> Domain for Just<T> {
    fn new() -> Self {
        Just {
            values: HashMap::new(),
            goals: Vec::new(),
        }
    }
}

impl<T> DomainType<T> for Just<T> {
    fn values_as_ref(&self) -> &HashMap<LVar, Val<T>> {
        &self.values
    }
    fn values_as_mut(&mut self) -> &mut HashMap<LVar, Val<T>> {
        &mut self.values
    }
}
