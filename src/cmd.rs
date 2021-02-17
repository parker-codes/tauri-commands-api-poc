use crate::app::Context;
use serde::de::DeserializeOwned;

pub trait CommandSet: DeserializeOwned {
    type State;
    fn execute(self, context: Context<Self::State>) -> Result<String, String>;
}

pub trait Executable {
    type State;
    fn execute(self, context: Context<Self::State>) -> Result<String, String>;
}
