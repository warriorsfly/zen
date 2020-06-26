use crate::handlers::{auth, user};
use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/auth")
                    .route("email_regist", web::post().to(auth::email_regist))
                    .route("login", web::post().to(auth::login))
                    .route("logout", web::get().to(auth::logout)),
            )
            .service(
                web::scope("/users"), // .route("/{id}", web::get().to(get_user))
                                      // .route("/{id}", web::put().to(update_user))
                                      // .route("/{id}", web::delete().to(delete_user))
                                      // .route("", web::get().to(get_users))
            ),
    );
}
