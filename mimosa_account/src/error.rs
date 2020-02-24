use crate::entity::response::Response;
use actix_web::{
    HttpResponse,http::StatusCode
};

pub struct ServiceError {
    pub status: StatusCode,
    pub body: Response<String>,
}

impl ServiceError {
    pub fn new(status: StatusCode, message: String) -> ServiceError {
        ServiceError {
            status,
            body: Response {
                message,
                data: String::new(),
            }
        }
    }

    pub fn response(&self) -> HttpResponse {
        HttpResponse::build(self.status).json(&self.body)
    }
}
