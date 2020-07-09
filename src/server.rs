use crate::{
    awc::add_awc, cache::add_cache, config::CONFIG, database::add_pool, routes::routes,
    state::new_state,
};
use actix_cors::Cors;
use actix_web::{middleware::Logger, App, HttpServer};

pub async fn server() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let data = new_state::<String>();

    let server = HttpServer::new(move || {
        App::new()
            // 添加缓存
            .configure(add_cache)
            // 添加awc
            .configure(add_awc)
            // 添加跨域
            .wrap(Cors::new().supports_credentials().finish())
            // 添加日志
            .wrap(Logger::default())
            // 连接数据库
            .configure(add_pool)
            // 添加状态
            .app_data(data.clone())
            // 注册路由
            .configure(routes)
    });

    server.bind(&CONFIG.server)?.run().await
}
