use crate::{
    auth::{create_jwt, JwtAccount},
    errors::ServiceError,
    schema::users,
};
use serde::Serialize;
use validator::Validate;

type Url = String;

#[derive(Deserialize, Insertable, Serialize, Validate)]
#[table_name = "users"]
pub struct NewUserData {
    pub username: String,
    pub email: String,
    pub password: String,
}

// TODO: remove clone when diesel will allow skipping fields
#[derive(Deserialize, AsChangeset, Default, Clone)]
#[table_name = "users"]
pub struct UpdateUserData {
    pub username: Option<String>,
    pub email: Option<String>,
    pub bio: Option<String>,
    pub avtar: Option<String>,
    pub password: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Queryable, Identifiable, Insertable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub bio: Option<String>,
    pub avtar: Option<Url>,
    #[serde(skip_serializing)]
    pub password: String,
}

#[derive(Serialize)]
pub struct AccountAuth<'a> {
    username: &'a str,
    email: &'a str,
    bio: Option<&'a str>,
    avtar: Option<&'a str>,
    token: String,
}

#[derive(Serialize)]
pub struct Profile {
    username: String,
    bio: Option<String>,
    avtar: Option<String>,
    following: bool,
}

impl User {
    pub fn to_user_auth(&self) -> Result<AccountAuth, ServiceError> {
        let token = create_jwt(JwtAccount::new(self.id, self.username.clone()))?;

        Ok(AccountAuth {
            username: &self.username,
            email: &self.email,
            bio: self.bio.as_ref().map(String::as_str),
            avtar: self.avtar.as_ref().map(String::as_str),
            token,
        })
    }

    pub fn to_profile(self, following: bool) -> Profile {
        Profile {
            username: self.username,
            bio: self.bio,
            avtar: self.avtar,
            following,
        }
    }
}
