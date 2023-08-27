use crate::application::functions::todo::CreatePayload;
use crate::domain::types::Date;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    Title,
    TodoAt,
}

impl ParseError {
    pub fn description(&self) -> String {
        match self {
            Self::Title => "required".to_string(),
            Self::TodoAt => {
                "optional, but if defined, must be a date on format YYYY-MM-DD".to_string()
            }
        }
    }
}

#[derive(Debug)]
pub struct CreateInput {
    pub title: Option<String>,
    pub description: Option<String>,
    pub todo_at: Option<String>,
}

impl CreateInput {
    pub fn parse(self) -> Result<CreatePayload, ParseError> {
        let title = self
            .title
            .filter(|t| !t.is_empty())
            .ok_or(ParseError::Title)?;

        let description = self.description.filter(|d| !d.is_empty());

        let todo_at = self
            .todo_at
            .map(|at| Date::parse_str(&at))
            .transpose()
            .map_err(|_| ParseError::TodoAt)?;

        Ok(CreatePayload {
            title,
            description,
            todo_at,
        })
    }
}

#[cfg(test)]
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

        assert!(none_title_payload.is_err_and(|e| e == super::ParseError::Title));

        let empty_title = super::CreateInput {
            title: Some("".to_string()),
            description: None,
            todo_at: None,
        };
        let empty_title_payload = empty_title.parse();

        assert!(empty_title_payload.is_err_and(|e| e == super::ParseError::Title));
    }

    #[test]
    fn parse_todo_at_fail() {
        let invalid_todo_at = super::CreateInput {
            title: Some("title".to_string()),
            description: None,
            todo_at: Some("2023-2023-2023".to_string()),
        };

        let invalid_todo_at_payload = invalid_todo_at.parse();

        assert!(invalid_todo_at_payload.is_err_and(|e| e == super::ParseError::TodoAt));
    }
}
