use crate::application::functions::todo::CreatePayload;
use time::{macros::format_description, Date};

#[derive(Debug)]
pub struct CreateTodoInput {
    pub title: Option<String>,
    pub description: Option<String>,
    pub todo_at: Option<String>,
}

impl CreateTodoInput {
    pub fn parse(self) -> Result<CreatePayload, String> {
        let title = self.title.ok_or("title is required".to_string())?;
        if title.is_empty() {
            return Err("title should not be empty".to_string());
        }

        let description = self.description.filter(|d| !d.is_empty());

        let todo_at = self
            .todo_at
            .map(|at| Date::parse(at.as_ref(), format_description!("[year]-[month]-[day]")))
            .transpose()
            .map_err(|_| "todo_at must be a date on the format YYYY-MM-DD".to_string())?;

        Ok(CreatePayload {
            title,
            description,
            todo_at,
        })
    }
}

mod tests {
    #[test]
    fn parse_success() {
        let expected_input = super::CreateTodoInput {
            title: Some("title".to_string()),
            description: Some("description".to_string()),
            todo_at: Some("2023-08-11".to_string()),
        };

        assert!(expected_input.parse().is_ok());
    }

    #[test]
    fn parse_empty_description_to_none() {
        let input = super::CreateTodoInput {
            title: Some("title".to_string()),
            description: Some("".to_string()),
            todo_at: None,
        };

        let payload = input.parse();

        assert!(payload.is_ok());
        assert_eq!(payload.unwrap().description, None);
    }

    #[test]
    fn parse_title_fail() {
        let none_title = super::CreateTodoInput {
            title: None,
            description: Some("description".to_string()),
            todo_at: None,
        };
        let none_title_payload = none_title.parse();

        assert!(none_title_payload.is_err());
        assert_eq!(
            none_title_payload.unwrap_err(),
            "title is required".to_string()
        );

        let empty_title = super::CreateTodoInput {
            title: Some("".to_string()),
            description: None,
            todo_at: None,
        };
        let empty_title_payload = empty_title.parse();

        assert!(empty_title_payload.is_err());
        assert_eq!(
            empty_title_payload.unwrap_err(),
            "title should not be empty".to_string()
        );
    }

    #[test]
    fn parse_todo_at_fail() {
        let invalid_todo_at = super::CreateTodoInput {
            title: Some("title".to_string()),
            description: None,
            todo_at: Some("2023-2023-2023".to_string()),
        };

        let invalid_todo_at_payload = invalid_todo_at.parse();

        assert!(invalid_todo_at_payload.is_err());
        assert_eq!(
            invalid_todo_at_payload.unwrap_err(),
            "todo_at must be a date on the format YYYY-MM-DD".to_string()
        );
    }
}
