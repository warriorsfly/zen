use crate::{
    auth::hash,
    database::PoolType,
    errors::ServiceError,
    helpers::respond_json,
    models::{NewUser, User, UserChange},
    repository,
    validate::validate,
};
use actix_web::web::{block, Data, Json, Path};
use rayon::prelude::*;
use serde::Serialize;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize)]
pub struct UsersResponse(pub Vec<User>);

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct CreateUserRequest {
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

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct UpdateUserRequest {
    #[validate(length(
        min = 6,
        message = "first_name is required and must be at least 6 characters"
    ))]
    pub username: String,

    #[validate(email(message = "email must be a valid email"))]
    pub email: String,
}

/// Get a user
// pub async fn get_user(
//     user_id: Path<Uuid>,
//     pool: Data<PoolType>,
// ) -> Result<Json<UserResponse>, ServiceError> {
//     let user = block(move || find(&pool, *user_id)).await?;
//     respond_json(user)
// }

/// Create a user
pub async fn create_user(
    pool: Data<PoolType>,
    params: Json<CreateUserRequest>,
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
    let user = block(move || repository::create_user(&pool, &new_user)).await?;
    respond_json(user)
}

// /// Update a user
// pub async fn update_user(
//     user_id: Path<Uuid>,
//     pool: Data<PoolType>,
//     params: Json<UpdateUserRequest>,
// ) -> Result<Json<UserResponse>, ServiceError> {
//     validate(&params)?;

//     // temporarily use the user's id for updated_at
//     // update when auth is added
//     let update_user = UserChange {
//         username: params.username.to_string(),
//         email: params.email.to_string(),
//     };
//     let user = block(move || repository::update_user(&pool, &update_user)).await?;
//     respond_json(user.into())
// }

/// Delete a user
// pub async fn delete_user(
//     user_id: Path<Uuid>,
//     pool: Data<PoolType>,
// ) -> Result<HttpResponse, ServiceError> {
//     block(move || delete(&pool, *user_id)).await?;
//     respond_ok()
// }

#[cfg(test)]
pub mod tests {
    // use super::*;
    // use crate::models::user::tests::create_user as model_create_user;
    // use crate::tests::helpers::tests::{get_data_pool, get_pool};

    // pub fn get_all_users() -> UsersResponse {
    //     let pool = get_pool();
    //     get_all(&pool).unwrap()
    // }

    // pub fn get_first_users_id() -> Uuid {
    //     get_all_users().0[0].id
    // }

    // #[actix_rt::test]
    // async fn it_gets_a_user() {
    //     let first_user = &get_all_users().0[0];
    //     let user_id: Path<Uuid> = get_first_users_id().into();
    //     let response = get_user(user_id, get_data_pool()).await.unwrap();
    //     assert_eq!(response.into_inner(), *first_user);
    // }

    // #[actix_rt::test]
    // async fn it_doesnt_find_a_user() {
    //     let uuid = Uuid::new_v4();
    //     let user_id: Path<Uuid> = uuid.into();
    //     let response = get_user(user_id, get_data_pool()).await;
    //     let expected_error = ServiceError::NotFound(format!("User {} not found", uuid.to_string()));
    //     assert!(response.is_err());
    //     assert_eq!(response.unwrap_err(), expected_error);
    // }

    // #[actix_rt::test]
    // async fn it_creates_a_user() {
    //     let params = Json(CreateUserRequest {
    //         first_name: "Satoshi".into(),
    //         last_name: "Nakamoto".into(),
    //         email: "satoshi@nakamotoinstitute.org".into(),
    //         password: "123456".into(),
    //     });
    //     let response = create_user(get_data_pool(), Json(params.clone()))
    //         .await
    //         .unwrap();
    //     assert_eq!(response.into_inner().username, params.first_name);
    // }

    // #[actix_rt::test]
    // async fn it_updates_a_user() {
    //     let first_user = &get_all_users().0[0];
    //     let user_id: Path<Uuid> = get_first_users_id().into();
    //     let params = Json(UpdateUserRequest {
    //         username: first_user.username.clone(),
    //         email: first_user.email.clone(),
    //     });
    //     let response = update_user(user_id, get_data_pool(), Json(params.clone()))
    //         .await
    //         .unwrap();
    //     assert_eq!(response.into_inner().user, params.first_name);
    // }

    // #[actix_rt::test]
    // async fn it_deletes_a_user() {
    //     let created = model_create_user();
    //     let user_id = created.unwrap().id;
    //     let user_id_path: Path<Uuid> = user_id.into();
    //     let user = find(&get_pool(), user_id);
    //     assert!(user.is_ok());
    //     delete_user(user_id_path, get_data_pool()).await.unwrap();
    //     let user = find(&get_pool(), user_id);
    //     assert!(user.is_err());
    // }
}
