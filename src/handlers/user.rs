use uuid::Uuid;
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct UserBaseResponse {
    pub id: Uuid,
    pub user_role: i32,
    pub register_source: i32,
    pub user_name: String,
    pub nick_name: String,
    pub gender: i32,
    pub birthday: chrono::NaiveDateTime,
    pub signature: String,
    pub mobile: String,
    pub email: String,
    pub avatar: String,
    pub avatar200: String,
    pub avatar_source: String,
    pub push_token: String,
}
