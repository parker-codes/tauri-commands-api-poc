use crate::cmd::{CommandSet, Executable};
use std::marker::PhantomData;

pub struct AppBuilder<T, U: CommandSet<T>> {
    user_data: Option<T>,
    cmd_set_handler: PhantomData<U>,
}

impl<T, U: CommandSet<T>> AppBuilder<T, U> {
    pub fn new() -> Self {
        Self {
            user_data: None,
            cmd_set_handler: PhantomData,
        }
    }

    pub fn user_data(mut self, user_data: T) -> Self {
        self.user_data = Some(user_data);
        self
    }

    pub fn build(self) -> App<T, U> {
        App {
            user_data: self.user_data.unwrap(),
            cmd_set_handler: self.cmd_set_handler,
            webview: "webview".to_string(),
        }
    }
}

pub struct App<T, U: CommandSet<T>> {
    user_data: T,
    cmd_set_handler: PhantomData<U>,
    webview: String,
}

impl<T, U: CommandSet<T>> App<T, U> {
    pub fn handle(self, arg: String) -> Result<String, String> {
        let context = Context {
            user_data: self.user_data,
            webview: "webview".to_string(),
        };

        // TODO: the end goal of U is to hint at what type the JSON should be deserialized into
        match serde_json::from_str::<U>(&arg) {
            Err(e) => Err(e.to_string()),
            Ok(cmd) => execute_cmd(cmd, context),
        }
    }
}

// wrapping the method call ensures that the user sees an impl error? there's probably a cleaner way to do this..
fn execute_cmd<T, U: CommandSet<T>>(cmd: U, context: Context<T>) -> Result<String, String> {
    cmd.execute(context)
}

pub struct Context<T> {
    user_data: T,
    webview: String,
}
