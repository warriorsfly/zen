use crate::entity::user::auth::LoginResultDTO;
use jsonwebtoken::{encode, Header, EncodingKey};
use chrono;

pub static KEY: [u8; 1675] = *include_bytes!("mimosa.key");
// pub static KEY:[u8; 16] = [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1];

static ONE_WEEK: i64 = 60 * 60 * 24 * 7;
#[derive(Serialize,Deserialize)]
pub struct UserToken {
    pub iat: i64,
    pub exp: i64,
    pub identity_type: i32,
    pub identifier: String,
    pub login_session: String,
}

impl UserToken {
    pub fn generate_token(login:LoginResultDTO)->String {
        let now = chrono::now();
        let payload = UserToken {
            iat:now,
            exp:now+ONE_WEEK,
            identity_type:login.identity_type,
            identifier:login.identifier,
            login_session:login.login_session,
        };
       encode(&Header::default(), &payload, &EncodingKey::from_secret(&KEY)).unwrap().to_string()
    }
}
