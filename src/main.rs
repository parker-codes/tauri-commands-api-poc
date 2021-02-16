fn main() {
    println!("Run tests");
}

fn handle(arg: &str) -> Result<String, String> {
    use cmd::Cmd;

    match serde_json::from_str::<Cmd>(arg) {
        Err(e) => Err(e.to_string()),
        Ok(cmd) => cmd.execute(),
    }
}

mod cmd {
    use serde::Deserialize;

    #[derive(Deserialize, Clone, Debug, PartialEq)]
    pub struct GetAllTodos;

    impl GetAllTodos {
        fn execute(&self) -> Result<String, String> {
            Ok("got 'em".to_string())
        }
    }

    #[derive(Deserialize, Clone, Debug, PartialEq)]
    pub struct CreateTodo {
        title: String,
    }

    impl CreateTodo {
        fn execute(&self) -> Result<String, String> {
            Ok("created".to_string())
        }
    }

    /*
        To be generated ..
    */

    #[derive(Deserialize, Clone, Debug, PartialEq)]
    #[serde(tag = "cmd", rename_all = "camelCase")]
    pub enum Cmd {
        GetAllTodos(GetAllTodos),
        CreateTodo(CreateTodo),
        Error, // to catch any issues
    }

    impl Cmd {
        pub fn execute(&self) -> Result<String, String> {
            match self {
                Self::GetAllTodos(cmd) => cmd.execute(),
                Self::CreateTodo(cmd) => cmd.execute(),
                Self::Error => Err("unknown command sent".to_string()),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn can_fire_command() {
        assert_eq!(
            crate::handle("{ \"cmd\": \"getAllTodos\" }"),
            Ok("got 'em".to_string())
        );
    }

    #[test]
    fn will_throw_missing_field_error() {
        assert_eq!(
            crate::handle("{ \"cmd\": \"createTodo\" }"),
            Err("missing field `title`".to_string())
        );
    }

    #[test]
    fn can_fire_command_with_args() {
        assert_eq!(
            crate::handle("{ \"cmd\": \"createTodo\", \"title\": \"Do laundry\" }"),
            Ok("created".to_string())
        );
    }

    #[test]
    fn will_throw_error_when_command_not_found() {
        assert!(crate::handle("{ \"cmd\": \"incorrect\" }").is_err());
    }
}
