// use crate::{config::CONFIG, database::add_pool, routes::routes};
use crate::{config::CONFIG, database::add_pool, routes::routes};
use actix_cors::Cors;
use actix_web::{App, Error, HttpServer, dev::ServiceRequest, middleware::Logger};
use actix_web_httpauth::{
    extractors::{
        bearer::{BearerAuth, Config},
        AuthenticationError,
    },
    middleware::HttpAuthentication,
};

async fn ok_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
    if credentials.token() == "mF_9.B5f-4.1JqM" {
        Ok(req)
    } else {
        let config = req
            .app_data::<Config>()
            .map(|data| data.clone())
            .unwrap_or_else(Default::default)
            .scope("urn:example:channel=HBO&urn:example:rating=G,PG-13");

        Err(AuthenticationError::from(config).into())
    }
}

pub async fn serv() -> std::io::Result<()> {
    env_logger::init();
    let server = HttpServer::new(move || {
        App::new()
            .wrap(HttpAuthentication::bearer(ok_validator))
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
