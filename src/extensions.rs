use std::fmt::Debug;
use std::result::Result;

pub type GenericResult<D> = Result<D, String>;
pub trait ResultGeneralize<T, E> where E: Debug {
    fn generalize(self) -> GenericResult<T>;
}

impl<T, E> ResultGeneralize<T, E> for Result<T, E> where E: Debug {
    fn generalize(self: Result<T, E>) -> GenericResult<T> {
        match self {
            Ok(value) => Ok(value),
            Err(error) => Err(format!("{:?}", error))
        }
    }
}

