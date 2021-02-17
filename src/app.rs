use crate::cmd::{CommandSet, Executable};
use std::marker::PhantomData;

pub struct AppBuilder<U: CommandSet> {
    user_data: Option<U::State>,
    cmd_set_handler: PhantomData<U>,
}

impl<U: CommandSet> AppBuilder<U> {
    pub fn new() -> Self {
        Self {
            user_data: None,
            cmd_set_handler: PhantomData,
        }
    }

    pub fn user_data(mut self, user_data: U::State) -> Self {
        self.user_data = Some(user_data);
        self
    }

    pub fn build(self) -> App<U> {
        App {
            user_data: self.user_data.unwrap(),
            cmd_set_handler: self.cmd_set_handler,
            webview: "webview".to_string(),
        }
    }
}

pub struct App<U: CommandSet> {
    pub user_data: U::State,
    cmd_set_handler: PhantomData<U>,
    webview: String,
}

impl<U: CommandSet> App<U> {
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
fn execute_cmd<U: CommandSet>(cmd: U, context: Context<U::State>) -> Result<String, String> {
    cmd.execute(context)
}

#[derive(Debug)]
pub struct Context<T> {
    pub user_data: T,
    pub webview: String,
}
