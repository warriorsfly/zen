use crate::{awc::add_awc, config::CONFIG, database::add_pool, routes::routes, state::new_state};
// use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger, App, HttpServer};

pub async fn serve() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let data = new_state::<String>();

    let server = HttpServer::new(move || {
        App::new()
            // 添加缓存
            // .configure(add_cache)
            // 添加awc
            .configure(add_awc)
            // // 添加跨域
            // .wrap(
            //     Cors::default()
            //         // .allowed_origin(&CONFIG.server)
            //         .allowed_methods(vec!["POST", "GET"])
            //         .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            //         .allowed_header(header::CONTENT_TYPE)
            //         .supports_credentials()
            //         .max_age(3600),
            // )
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
