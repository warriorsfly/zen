#[cfg(test)]
pub mod tests {

    use crate::{
        config::CONFIG,
        constants,
        database::add_pool,
        handlers::auth::{LoginData, LoginResponse},
        routes::routes,
        state::{new_state, AppState},
    };
    use actix_web::{dev::ServiceResponse, test, web::Data, App};
    use serde::Serialize;
    use zen_database::{init_pool, DatabaseConnectionPool};

    /// Helper for HTTP GET integration tests
    pub async fn test_get(route: &str) -> ServiceResponse {
        let login_request = LoginData {
            email: "warriorsfly@gmail.com".into(),
            password: "123456".into(),
        };

        let mut app = test::init_service(
            App::new()
                .wrap(middleware::Authentication)
                // .configure(add_cache)
                .app_data(app_state())
                .configure(add_pool)
                .configure(routes),
        )
        .await;

        let json: LoginResponse = test::read_response_json(
            &mut app,
            test::TestRequest::post()
                .set_json(&login_request)
                .uri("/api/auth/login")
                .to_request(),
        )
        .await;

        // let cookie = response.response().headers().get(constants::AUTHORIZATION);

        test::call_service(
            &mut app,
            test::TestRequest::get()
                .header(constants::AUTHORIZATION, json.token)
                .uri(route)
                .to_request(),
        )
        .await
    }

    /// Helper for HTTP GET integration tests
    pub async fn test_post<T: Serialize>(route: &str, params: T) -> ServiceResponse {
        let login_request = LoginData {
            email: "warriorsfly@gmail.com".into(),
            password: "123456".into(),
        };

        let mut app = test::init_service(
            App::new()
                // .configure(add_cache)
                .app_data(app_state())
                .configure(add_pool)
                .configure(routes),
        )
        .await;

        let json: LoginResponse = test::read_response_json(
            &mut app,
            test::TestRequest::post()
                .set_json(&login_request)
                .uri("/api/auth/login")
                .to_request(),
        )
        .await;

        test::call_service(
            &mut app,
            test::TestRequest::post()
                .set_json(&params)
                .header(constants::AUTHORIZATION, json.token)
                .uri(route)
                .to_request(),
        )
        .await
    }

    pub async fn assert_get(route: &str) -> ServiceResponse {
        let response = test_get(route).await;
        assert!(response.status().is_success());
        response
    }

    pub async fn assert_post<T: Serialize>(route: &str, params: T) -> ServiceResponse {
        let response = test_post(route, params).await;
        assert!(response.status().is_success());
        response
    }

    // Mock applicate sql connection pool
    pub fn get_pool() -> DatabaseConnectionPool {
        init_pool(&CONFIG.database_url).unwrap()
    }

    /// Returns a r2d2 Pooled Connection wrappedn in Actix Application Data
    pub fn get_data_pool() -> Data<DatabaseConnectionPool> {
        Data::new(get_pool())
    }

    pub async fn login() -> ServiceResponse {
        let login_request = LoginData {
            email: "warriorsfly@gmail.com".into(),
            password: "123456".into(),
        };

        let mut app = test::init_service(
            App::new()
                .wrap(middleware::Authentication)
                .configure(add_pool)
                .configure(routes),
        )
        .await;

        test::call_service(
            &mut app,
            test::TestRequest::post()
                .set_json(&login_request)
                .uri("api/auth/login")
                .to_request(),
        )
        .await
    }
    // Mock applicate state
    pub fn app_state() -> AppState<'static, String> {
        new_state::<String>()
    }
}
