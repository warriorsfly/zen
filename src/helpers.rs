use actix_web::{
    body::Body,
    web::{HttpResponse, Json},
};
use serde::Serialize;

use crate::errors::ZenError;
/// 快速组装Ok/Json response
pub fn respond_json<T>(data: T) -> Result<Json<T>, ZenError>
where
    T: Serialize,
{
    Ok(Json(data))
}
///
pub fn respond_ok() -> Result<HttpResponse<Body>, ZenError> {
    Ok(HttpResponse::ok())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    #[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
    pub struct TestResponse {
        pub first_name: String,
    }
    #[test]
    fn it_responds_json() {
        let response = TestResponse {
            first_name: "Zhang".into(),
        };

        let result = respond_json(response.clone());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().into_inner(), response);
    }

    #[test]
    fn it_responds_ok() {
        let result = respond_ok();
        assert!(result.is_ok());
    }
}
