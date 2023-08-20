use crate::application::functions::todo::UpdatePayload;
use crate::domain::types::{Date, Id};

#[derive(Clone, Debug, PartialEq)]
pub enum ParseError {
    Id,
    Title,
    TodoAt,
}

impl ParseError {
    pub fn description(&self) -> String {
        match self {
            Self::Id => "required and it should be a valid uuid".to_string(),
            Self::Title => "required".to_string(),
            Self::TodoAt => "optional, if defined, must be a date on format YYYY-MM-DD".to_string(),
        }
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Id => write!(f, "Id"),
            Self::Title => write!(f, "Title"),
            Self::TodoAt => write!(f, "TodoAt"),
        }
    }
}

#[derive(Debug)]
pub struct UpdateInput {
    pub id: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub todo_at: Option<String>,
}

impl UpdateInput {
    pub fn parse(self) -> Result<UpdatePayload, ParseError> {
        let id = self
            .id
            .map(|id| Id::parse_str(&id))
            .ok_or(ParseError::Id)?
            .map_err(|_| ParseError::Id)?;

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

        Ok(UpdatePayload {
            id,
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
        let input = super::UpdateInput {
            id: Some(super::Id::new().as_string()),
            title: Some("title".to_string()),
            description: Some("description".to_string()),
            todo_at: Some("2023-08-12".to_string()),
        };

        assert!(input.parse().is_ok());
    }

    #[test]
    fn parse_id_fail() {
        let none_id = super::UpdateInput {
            id: None,
            title: Some("title".to_string()),
            description: Some("description".to_string()),
            todo_at: Some("2023-08-12".to_string()),
        };
        let none_id_payload = none_id.parse();

        assert!(none_id_payload.is_err_and(|e| e == super::ParseError::Id));

        let invalid_id = super::UpdateInput {
            id: Some("invalid-id".to_string()),
            title: Some("title".to_string()),
            description: None,
            todo_at: None,
        };
        let invalid_id_payload = invalid_id.parse();

        assert!(invalid_id_payload.is_err_and(|e| e == super::ParseError::Id));
    }

    #[test]
    fn parse_title_fail() {
        let none_title = super::UpdateInput {
            id: Some(super::Id::new().as_string()),
            title: None,
            description: None,
            todo_at: None,
        };
        let none_title_payload = none_title.parse();

        assert!(none_title_payload.is_err_and(|e| e == super::ParseError::Title));

        let empty_title = super::UpdateInput {
            id: Some(super::Id::new().as_string()),
            title: Some("".to_string()),
            description: None,
            todo_at: None,
        };
        let empty_title_payload = empty_title.parse();

        assert!(empty_title_payload.is_err_and(|e| e == super::ParseError::Title));
    }

    #[test]
    fn parse_todo_at_fail() {
        let invalid_todo_at = super::UpdateInput {
            id: Some(super::Id::new().as_string()),
            title: Some("title".to_string()),
            description: None,
            todo_at: Some("todo_at".to_string()),
        };
        let invalid_todo_at_payload = invalid_todo_at.parse();

        assert!(invalid_todo_at_payload.is_err_and(|e| e == super::ParseError::TodoAt))
    }
}
