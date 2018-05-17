use super::Error;

#[derive(Debug, Default)]
pub struct ValidationResult {
    pub errors: Vec<Error>,
}

