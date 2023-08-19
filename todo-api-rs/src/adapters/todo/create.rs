use crate::application::functions::todo::CreatePayload;
use crate::domain::types::Date;

#[derive(Debug)]
pub struct CreateInput {
    pub title: Option<String>,
    pub description: Option<String>,
    pub todo_at: Option<String>,
}

impl CreateInput {
    pub fn parse(self) -> Result<CreatePayload, String> {
        let title = self
            .title
            .filter(|t| !t.is_empty())
            .ok_or("title is required".to_string())?;

        let description = self.description.filter(|d| !d.is_empty());

        let todo_at = self
            .todo_at
            .map(|at| Date::parse(&at))
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
        let expected_input = super::CreateInput {
            title: Some("title".to_string()),
            description: Some("description".to_string()),
            todo_at: Some("2023-08-11".to_string()),
        };

        assert!(expected_input.parse().is_ok());
    }

    #[test]
    fn parse_empty_description_to_none() {
        let input = super::CreateInput {
            title: Some("title".to_string()),
            description: Some("".to_string()),
            todo_at: None,
        };
        let payload = input.parse();

        assert!(payload.is_ok_and(|p| p.description.is_none()));
    }

    #[test]
    fn parse_title_fail() {
        let none_title = super::CreateInput {
            title: None,
            description: Some("description".to_string()),
            todo_at: None,
        };
        let none_title_payload = none_title.parse();

        assert!(none_title_payload.is_err_and(|e| e == "title is required"));

        let empty_title = super::CreateInput {
            title: Some("".to_string()),
            description: None,
            todo_at: None,
        };
        let empty_title_payload = empty_title.parse();

        assert!(empty_title_payload.is_err_and(|e| e == "title is required"));
    }

    #[test]
    fn parse_todo_at_fail() {
        let invalid_todo_at = super::CreateInput {
            title: Some("title".to_string()),
            description: None,
            todo_at: Some("2023-2023-2023".to_string()),
        };

        let invalid_todo_at_payload = invalid_todo_at.parse();

        assert!(invalid_todo_at_payload
            .is_err_and(|e| e == "todo_at must be a date on the format YYYY-MM-DD"));
    }
}
