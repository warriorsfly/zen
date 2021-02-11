use crate::errors::ServError;
use actix_web::web::Json;
use validator::{Validate, ValidationErrors};

pub fn validate<T>(params: &Json<T>) -> Result<(), ServError>
where
    T: Validate,
{
    match params.validate() {
        Ok(()) => Ok(()),
        Err(error) => Err(ServError::ValidationError(collect_errors(error))),
    }
}

/// Collect ValidationErrors and return a vector of the messages
/// Adds a default_error when none is supplied
fn collect_errors(error: ValidationErrors) -> Vec<String> {
    error
        .field_errors()
        .into_iter()
        .map(|e| {
            let default_error = format!("{} is required", e.0);
            e.1[0]
                .message
                .as_ref()
                .unwrap_or(&std::borrow::Cow::Owned(default_error))
                .to_string()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    #[derive(Debug, Deserialize, Serialize, Validate)]
    struct TestRequest {
        #[validate(length(
            min = 3,
            message = "first_name is required and must be at least 3 characters"
        ))]
        pub first_name: String,
    }

    fn get_test_request() -> TestRequest {
        let json = json!({"first_name":"a"});
        serde_json::from_value::<TestRequest>(json).unwrap()
    }

    #[test]
    fn it_collect_errors() {
        let request = get_test_request();
        let errors = request.validate().unwrap_err();
        let response = collect_errors(errors);
        assert!(response.len() > 0);
    }
}
