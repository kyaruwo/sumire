use validator::ValidationError;

pub fn empty_string(field: &String) -> Result<(), ValidationError> {
    if field.trim().is_empty() {
        return Err(ValidationError::new("empty"));
    }
    Ok(())
}
