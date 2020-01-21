use crate::api::*;
use actix_web::web;

pub fn config_services(
    cfg: &mut web::ServiceConfig,
)
{
    info!("Configurating routes...");
    cfg.service(
        web::scope("/api")
            // .service(ping_controller::ping)
            .service(
                web::scope("/auth")
                    .service(
                        web::resource("/signup")
                            .route(web::post().to(account_controller::signup))
                    )
                    .service(
                        web::resource("/login")
                            .route(web::post().to(account_controller::login))
                    )
                    .service(
                        web::resource("/logout")
                            .route(web::post().to(account_controller::logout))
                    )
            )
    );
}
