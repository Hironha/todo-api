use crate::adapters::dtos::ParsableInput;
use crate::domain::types::Id;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    Id,
}

impl ParseError {
    pub fn description(&self) -> String {
        let description = match self {
            Self::Id => "required",
        };
        description.into()
    }
}

#[derive(Debug)]
pub struct Input {
    pub id: Id,
}

#[derive(Debug)]
pub struct InputSchema {
    pub id: Option<String>,
}

impl ParsableInput<Input, ParseError> for InputSchema {
    fn parse(self) -> Result<Input, ParseError> {
        let id = self
            .id
            .map(|id| Id::parse_str(&id))
            .ok_or(ParseError::Id)?
            .map_err(|_| ParseError::Id)?;

        Ok(Input { id })
    }
}

pub struct Output;

impl Output {
    pub const fn new() -> Self {
        Output {}
    }
}

#[cfg(test)]
mod test {
    use crate::adapters::dtos::ParsableInput;

    #[test]
    fn parse_success() {
        let input_schema = super::InputSchema {
            id: Some(super::Id::new().as_string()),
        };

        assert!(input_schema.parse().is_ok())
    }

    #[test]
    fn parse_fail() {
        let none_id_schema = super::InputSchema { id: None };
        let none_id_input = none_id_schema.parse();

        assert!(none_id_input.is_err());
        assert_eq!(none_id_input.unwrap_err(), super::ParseError::Id);

        let invalid_id_schema = super::InputSchema {
            id: Some("invalid-id".to_string()),
        };
        let invalid_id_input = invalid_id_schema.parse();

        assert!(invalid_id_input.is_err());
        assert_eq!(invalid_id_input.unwrap_err(), super::ParseError::Id);
    }
}
