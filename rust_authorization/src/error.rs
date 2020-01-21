use crate::models::response::ResponseBody;
use actix_web::{
    HttpResponse,http::StatusCode
};

pub struct ServiceError {
    pub status: StatusCode,
    pub body: ResponseBody<String>,
}

impl ServiceError {
    pub fn new(status: StatusCode, message: String) -> ServiceError {
        ServiceError {
            status,
            body: ResponseBody {
                message,
                data: String::new(),
            }
        }
    }

    pub fn response(&self) -> HttpResponse {
        HttpResponse::build(self.status).json(&self.body)
    }
}
