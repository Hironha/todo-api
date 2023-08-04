use serde::Deserialize;

use crate::application::functions::todo::CreatePayload;

#[derive(Debug, Deserialize)]
pub struct CreateTodoInput {
    title: Option<String>,
    description: Option<String>,
    #[serde(rename(deserialize = "todoAt"))]
    todo_at: Option<String>,
}

impl CreateTodoInput {
    pub fn into_payload(self) -> Result<CreatePayload, String> {
        let title = self.title.ok_or("title is required".to_string())?;
        if title.is_empty() {
            return Err("title should not be empty".to_string());
        }

        let description = self.description.filter(|d| !d.is_empty());

        Ok(CreatePayload {
            title,
            description,
            todo_at: self.todo_at,
        })
    }
}

mod tests {
    #[test]
    fn parse_success() {
        let expected_input = super::CreateTodoInput {
            title: Some("title".to_string()),
            description: Some("description".to_string()),
            todo_at: Some("todo_at".to_string()),
        };

        assert!(expected_input.into_payload().is_ok());
    }

    #[test]
    fn parse_empty_description_to_none() {
        let input = super::CreateTodoInput {
            title: Some("title".to_string()),
            description: Some("".to_string()),
            todo_at: None,
        };

        let payload = input.into_payload();

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
        let none_title_payload = none_title.into_payload();

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
        let empty_title_payload = empty_title.into_payload();

        assert!(empty_title_payload.is_err());
        assert_eq!(
            empty_title_payload.unwrap_err(),
            "title should not be empty".to_string()
        );
    }
}
