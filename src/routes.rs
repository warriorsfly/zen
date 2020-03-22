use crate::handlers::auth::{wx_access, wx_login};
use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api").service(
            web::scope("/auth")
                .route("wx-login/{code}", web::get().to(wx_login))
                .route("wx-access-token", web::get().to(wx_access)),
        ),
    );
}
