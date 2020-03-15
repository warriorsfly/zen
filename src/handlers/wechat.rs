use crate::{config::CONFIG, database::PoolType, errors::ServiceError, helpers::respond_json};
use actix_identity::Identity;
use actix_web::{
    self,
    client::Client,
    web::{Data, Json, Path},
};
#[derive(Debug, Deserialize, Serialize)]
pub struct WxSessionResponse {
    pub openid: Option<String>,
    pub session_key: Option<String>,
    pub unionid: Option<String>,
    pub errcode: Option<i32>,
    pub errmsg: Option<String>,
}

/// 微信小程序登录接口
pub async fn wx_login(
    id: Identity,
    pool: Data<PoolType>,
    jscode: Path<String>,
    client: Data<Client>,
) -> Result<Json<WxSessionResponse>, ServiceError> {
    let url =format!("https://api.weixin.qq.com/sns/jscode2session?appid={appid}&secret={secret}&js_code={code}&grant_type=authorization_code",appid=&CONFIG.wechat_appid,secret=&CONFIG.wechat_secret,code=jscode);
    let body = client
        .get(url)
        .send()
        .await
        .map_err(|err| ServiceError::BadRequest(err.to_string()))?
        .body()
        .await
        .map_err(|err| ServiceError::BadRequest(err.to_string()))?;

    let res: WxSessionResponse =
        serde_json::from_slice(&body).map_err(|err| ServiceError::BadRequest(err.to_string()))?;

    // response errorcode==None means  the right openid and session_key
    // find in redis session,if exists,return it
    // if doesn't exist,query entity in user_auth,where identity_type=5 and identifier=res.openid
    // if no result,insert one,identity_type=5 and identifier=res.openid,login_session=res.session_key,
    //   insert one in user_base
    //return user_auth entity,create jwt,create redis session
    if res.errcode == None {
        // find_by_auth(&pool,res.openid,"".)
    }
    respond_json(res)
}
