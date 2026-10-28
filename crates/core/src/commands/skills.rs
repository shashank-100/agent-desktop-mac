use crate::AppError;
use serde_json::{Value, json};

pub struct GetArgs {
    pub name: String,
    pub full: bool,
    pub reference: Option<String>,
}

pub fn list() -> Result<Value, AppError> {
    Ok(json!({"skills": []}))
}

pub fn get(args: GetArgs) -> Result<Value, AppError> {
    Err(AppError::invalid_input(format!(
        "No bundled skills are available: {}",
        args.name
    )))
}

pub fn path() -> Result<Value, AppError> {
    Ok(json!({"location": null, "available": []}))
}
