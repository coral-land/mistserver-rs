use crate::Result;

pub trait ApiController<T> {
    fn get(&self, identifier: String) -> Result<T>;
}
