use crate::handlers::{article, auth, user};
use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/auth")
                    .route("signup", web::post().to(auth::signup))
                    .route("login", web::post().to(auth::login)),
            )
            .service(
                web::scope("/users").route("/{id}", web::get().to(user::get_user)), // .route("/{id}", web::put().to(update_user))
                                                                                    // .route("/{id}", web::delete().to(delete_user))
                                                                                    // .route("", web::get().to(get_users))
            )
            .service(
                web::scope("/articles").route("", web::post().to(article::post_article)), // .route("/{id}", web::put().to(update_user))
                                                                                          // .route("/{id}", web::delete().to(delete_user))
                                                                                          // .route("", web::get().to(get_users))
            ),
    );
}
