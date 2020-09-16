use crate::{
    auth::{create_jwt, hash, PrivateClaim},
    database::DatabaseConnectionPool,
    db,
    errors::ServiceError,
    helpers::respond_json,
    models::{NewUser, User},
    validate::validate,
};
use actix_web::web::{block, Data, Json};
use serde::Serialize;
use validator::Validate;

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct SignupData {
    #[validate(length(
        min = 6,
        message = "last_name is required and must be at least 6 characters"
    ))]
    pub username: String,

    #[validate(email(message = "email must be a valid email"))]
    pub email: String,

    #[validate(length(
        min = 6,
        message = "password is required and must be at least 6 characters"
    ))]
    pub password: String,
}

/// 邮箱注册用户
pub async fn signup(
    pool: Data<DatabaseConnectionPool>,
    params: Json<SignupData>,
) -> Result<Json<User>, ServiceError> {
    validate(&params)?;
    let pass = hash(&params.password);
    let new_user = NewUser {
        username: params.username.clone(),
        email: params.email.clone(),
        password: pass,
        bio: None,
        avatar: None,
    };
    let user = block(move || db::create_user(&pool, &new_user)).await?;
    respond_json(user)
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct LoginData {
    #[validate(email(message = "email must be a valid email"))]
    pub email: String,

    #[validate(length(
        min = 6,
        message = "password is required and must be at least 6 characters"
    ))]
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginResponse {
    pub user: User,

    pub token: String,
}

/// Login a user
/// Create and remember their JWT
pub async fn login(
    pool: Data<DatabaseConnectionPool>,
    params: Json<LoginData>,
) -> Result<Json<LoginResponse>, ServiceError> {
    validate(&params)?;

    // Validate that the email + hashed password matches
    let hashed = hash(&params.password);
    let user = block(move || db::find_by_email(&pool, &params.email, &hashed)).await?;

    // Create a JWT
    let private_claim = PrivateClaim::new(user.id);
    let jwt = create_jwt(private_claim)?;
    let response = LoginResponse { user, token: jwt };
    respond_json(response)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::tests::helpers::tests::get_data_pool;
    use actix_web::{test, FromRequest};

    async fn get_private_claim() -> PrivateClaim {
        let (request, mut payload) =
            test::TestRequest::with_header("content-type", "application/json").to_http_parts();

        let claim = Option::<PrivateClaim>::from_request(&request, &mut payload)
            .await
            .unwrap()
            .unwrap();
        claim
    }

    async fn login_user() -> Result<Json<LoginResponse>, ServiceError> {
        let params = LoginData {
            email: "warriorsfly@gmail.com".into(),
            password: "123456".into(),
        };
        login(get_data_pool(), Json(params)).await
    }

    #[actix_rt::test]
    async fn it_logs_a_user_in() {
        let response = login_user().await;
        assert!(response.is_ok());
    }
}
