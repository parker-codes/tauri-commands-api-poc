use crate::app::Context;

pub trait CommandSet {
    fn execute(&self, context: Context) -> Result<String, String>;
}

pub trait Executable {
    fn execute(self, context: Context) -> Result<String, String>;
}
