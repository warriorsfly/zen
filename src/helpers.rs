use crate::errors::ServiceError;
use actix_web::{
    body::Body,
    web::{HttpResponse, Json},
};
use serde::Serialize;
/// 快速组装Ok/Json response
pub fn respond_json<T>(data: T) -> Result<Json<T>, ServiceError>
where
    T: Serialize,
{
    Ok(Json(data))
}
/// 快速组装Ok/Ok response
pub fn respond_ok() -> Result<HttpResponse, ServiceError> {
    Ok(HttpResponse::Ok().body(Body::Empty))
}

#[cfg(test)]
mod tests {
    use super::*;

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
