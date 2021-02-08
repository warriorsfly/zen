use actix_web::web;

use crate::handlers::{graphiql_handler, graphql, playground_handler};
pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(web::post().to(graphql))
            .route(web::get().to(graphql)),
    )
    .service(web::resource("/playground").route(web::get().to(playground_handler)))
    .service(web::resource("/graphiql").route(web::get().to(graphiql_handler)));
}
