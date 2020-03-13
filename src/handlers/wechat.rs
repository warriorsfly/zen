use crate::helpers::respond_json;
use crate::{config::CONFIG, errors::ServiceError};
use actix_web::{
    self,
    client::Client,
    web::{BytesMut, Data, Json, Path},
    Error,
};
#[derive(Debug, Deserialize, Serialize)]
pub struct WxLoginResponse {
    pub openid: String,
    pub session_key: String,
    pub unionid: String,
    pub errcode: i32,
    pub errmsg: String,
}

/// 微信小程序登录接口
pub async fn wx_login(
    jscode: Path<String>,
    client: Data<Client>,
) -> Result<Json<WxLoginResponse>, ServiceError> {
    let config = &CONFIG;
    let url =format!("https://api.weixin.qq.com/sns/jscode2session?appid={appid}&secret={secret}&js_code={code}&grant_type=authorization_code",appid=config.wechat_appid,secret=config.wechat_secret,code=jscode);
    let body = client
        .get(url)
        .send()
        .await
        .map_err(|err| ServiceError::BadRequest(err.to_string()))?
        .body()
        .await
        .map_err(|err| ServiceError::BadRequest(err.to_string()))?;

    let json: WxLoginResponse = serde_json::from_slice(&body).unwrap();

    respond_json(json)
}
