use actix_web::{client::Client, web::ServiceConfig};

/// Add awc to actix data if the URL is set
pub fn add_awc(cfg: &mut ServiceConfig) {
    let client = Client::default();
    cfg.data(client);
}
