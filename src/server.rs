use crate::{config::CONFIG, database::add_pool, routes::routes};
use actix_cors::Cors;
use actix_files::Files;
use actix_web::{middleware::Logger, App, HttpServer};

pub async fn serv() -> std::io::Result<()> {
    env_logger::init();
    let server = HttpServer::new(move || {
        App::new()
            // .wrap(HttpAuthentication::bearer(bearer_validator))
            .service(Files::new("/static", ".").prefer_utf8(true))
            // 添加缓存
            // .configure(add_cache)
            // 添加awc
            // .configure(add_awc)
            // 添加跨域
            .wrap(Cors::permissive())
            // 添加日志
            .wrap(Logger::default())
            // 连接数据库
            .configure(add_pool)
            // 添加状态
            // .app_data(data.clone())
            // 注册路由
            .configure(routes)
    });

    server.bind(&CONFIG.server)?.run().await
}
