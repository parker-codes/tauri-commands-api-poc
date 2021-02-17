use crate::app::Context;
use serde::de::DeserializeOwned;

pub trait CommandSet<T>: DeserializeOwned {
    fn execute(self, context: Context<T>) -> Result<String, String>;
}

pub trait Executable<T> {
    fn execute(self, context: Context<T>) -> Result<String, String>;
}
