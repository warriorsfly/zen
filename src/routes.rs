use crate::handlers::auth::wx_login;
use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(web::scope("/auth").route("wx-login/{code}", web::get().to(wx_login))),
    );
}
