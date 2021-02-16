use crate::cmd::{CommandSet, Executable};

pub struct AppBuilder<T, U: CommandSet> {
    user_data: Option<T>,
    cmd_set_handler: Option<U>,
}

impl<T, U: CommandSet> AppBuilder {
    pub fn new() -> Self {
        Self {
            user_data: None,
            cmd_set_handler: None,
        }
    }

    pub fn user_data(mut self, user_data: T) -> Self {
        self.user_data = Some(user_data);
        self
    }

    pub fn cmd_set_handler(mut self, command_set: U) -> Self {
        self.user_data = Some(command_set);
        self
    }

    pub fn build(self) -> App {
        App {
            user_data: self.user_data,
            cmd_set_handler: self.command_set_handler,
            webview: "webview".to_string(),
        }
    }
}

pub struct App<T, U: CommandSet> {
    user_data: T,
    cmd_set_handler: U,
    webview: String,
}

impl App {
    pub fn handle(self, arg: &str) -> Result<String, String> {
        let context = Context {
            user_data: self.user_data,
            webview: "webview".to_string(),
        };

        // TODO: the end goal of U is to hint at what type the JSON should be deserialized into
        match serde_json::from_str::<U>(arg) {
            Err(e) => Err(e.to_string()),
            Ok(cmd) => execute_cmd(cmd, context),
        }
    }
}

// wrapping the method call ensures that the user sees an impl error? there's probably a cleaner way to do this..
fn execute_cmd<U: CommandSet>(cmd: U, context: Context) -> Result<String, String> {
    cmd.execute(context)
}

pub struct Context<T> {
    user_data: T,
    webview: String,
}
