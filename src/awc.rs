// use crate::config::CONFIG;
// use actix_web::web::ServiceConfig;
// use actc

// /// Add awc to actix data if the URL is set
// pub fn add_awc(cfg: &mut ServiceConfig) {
//     if !(!&CONFIG.wechat_appid.is_empty() && !&CONFIG.wechat_secret.is_empty()) {
//         return;
//     }
//     // Start a new supervisor with redis actor
//     let client = Client::default();
//     cfg.data(client);
// }
