use crate::entity::user::LoginResultDTO;
use jsonwebtoken::Header;
use time::PrimitiveDateTime;

pub static KEY: [u8; 16] = [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1];
// static KEY: [u8; 16] = *include_bytes!("../secret.key");

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
    pub fn generate_token(login:LoginResultDTO)->String{
        let now:i64 = PrimitiveDateTime::now().timestamp();
        let payload:UserToken = UserToken{
            iat:now,
            exp:now+ONE_WEEK,
            identity_type:login.identity_type,
            identifier:login.identifier,
            login_session:login.login_session,
        };

        jsonwebtoken::encode(&Header::default(), &payload, &KEY).unwrap()
    }
}
