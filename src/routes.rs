use crate::handlers::user::create_user;
use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api").service(
            web::scope("/users")
                // .route("/{id}", web::get().to(get_user))
                // .route("/{id}", web::put().to(update_user))
                // .route("/{id}", web::delete().to(delete_user))
                // .route("", web::get().to(get_users))
                .route("", web::post().to(create_user)),
        ),
    );
}
