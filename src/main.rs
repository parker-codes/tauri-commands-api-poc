mod app;
mod cmd;

use app::Context;
use cmd::{CommandSet, Executable};
use serde::Deserialize;

// user-defined app state
struct State {
    status: String,
}

// user defined commands (eventually will be wrapped in macro)

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct GetAllTodos;

impl Executable for GetAllTodos {
    fn execute(self) -> Result<String, String> {
        Ok("got 'em".to_string())
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct CreateTodo {
    title: String,
}

impl Executable for CreateTodo {
    fn execute(self) -> Result<String, String> {
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
    fn execute(&self, context: Context) -> Result<String, String> {
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

    fn setup() -> App {
        let app_state = State {
            status: "idle".to_string(),
        };

        let app = AppBuilder::new()
            .user_data(app_state)
            .cmd_set_handler::<Commands>()
            .build();

        app
    }

    #[test]
    fn can_fire_command() {
        let app = setup();

        assert_eq!(
            app.handle("{ \"cmd\": \"getAllTodos\" }"),
            Ok("got 'em".to_string())
        );
    }

    #[test]
    fn will_throw_missing_field_error() {
        let app = setup();

        assert_eq!(
            app.handle("{ \"cmd\": \"createTodo\" }"),
            Err("missing field `title`".to_string())
        );
    }

    #[test]
    fn can_fire_command_with_args() {
        let app = setup();

        assert_eq!(
            app.handle("{ \"cmd\": \"createTodo\", \"title\": \"Do laundry\" }"),
            Ok("created".to_string())
        );
    }

    #[test]
    fn will_throw_error_when_command_not_found() {
        let app = setup();

        assert!(app.handle("{ \"cmd\": \"incorrect\" }").is_err());
    }

    // TODO: test that impl warning is shown
}
