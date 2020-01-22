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
                    .service(account_controller::signup)
                    .service(account_controller::login)
                    .service(account_controller::logout)
            )
    );
}
