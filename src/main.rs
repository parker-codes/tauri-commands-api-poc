mod app;
mod cmd;

use app::Context;
use cmd::{CommandSet, Executable};
use serde::Deserialize;

// user-defined app state
#[derive(Debug)]
pub struct State {
    status: String,
}

// user defined commands (eventually will be wrapped in macro)

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct GetAllTodos;

impl Executable for GetAllTodos {
    type State = State;
    fn execute(self, context: Context<State>) -> Result<String, String> {
        dbg!(&context);

        if context.user_data.status == "error" {
            return Err("uh oh!".to_string());
        }

        Ok("got 'em".to_string())
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct CreateTodo {
    title: String,
}

impl Executable for CreateTodo {
    type State = State;
    fn execute(self, context: Context<State>) -> Result<String, String> {
        dbg!(&context.webview);

        Ok("created".to_string())
    }
}

/*
    To be generated ..
*/

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Commands {
    GetAllTodos(GetAllTodos),
    CreateTodo(CreateTodo),
    Error, // to catch any issues
}

impl CommandSet for Commands {
    type State = State;
    fn execute(self, context: Context<State>) -> Result<String, String> {
        match self {
            Self::GetAllTodos(cmd) => cmd.execute(context),
            Self::CreateTodo(cmd) => cmd.execute(context),
            Self::Error => Err("unknown command sent".to_string()),
        }
    }
}

fn main() {
    println!("Run tests");
}

#[cfg(test)]
mod tests {
    use crate::app::{App, AppBuilder};
    use crate::{Commands, State};

    fn setup() -> App<Commands> {
        let app_state = State {
            status: "idle".to_string(),
        };

        let app = AppBuilder::new().user_data(app_state).build();

        app
    }

    #[test]
    fn can_fire_command() {
        let app = setup();

        assert_eq!(
            app.handle("{ \"cmd\": \"getAllTodos\" }".to_string()),
            Ok("got 'em".to_string())
        );
    }

    #[test]
    fn will_throw_missing_field_error() {
        let app = setup();

        assert_eq!(
            app.handle("{ \"cmd\": \"createTodo\" }".to_string()),
            Err("missing field `title`".to_string())
        );
    }

    #[test]
    fn can_fire_command_with_args() {
        let app = setup();

        assert_eq!(
            app.handle("{ \"cmd\": \"createTodo\", \"title\": \"Do laundry\" }".to_string()),
            Ok("created".to_string())
        );
    }

    #[test]
    fn will_throw_error_when_command_not_found() {
        let app = setup();

        assert!(app
            .handle("{ \"cmd\": \"incorrect\" }".to_string())
            .is_err());
    }

    #[test]
    fn can_access_user_data_through_context() {
        let mut app = setup();

        // NOTE: user_data must be public for this test - is that okay to expose?
        app.user_data.status = "error".to_string();

        assert_eq!(
            app.handle("{ \"cmd\": \"getAllTodos\" }".to_string()),
            Err("uh oh!".to_string())
        );
    }
}
