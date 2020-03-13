use crate::{config::CONFIG, errors::ServiceError};
use actix_web::{client::Client, web::Path};

pub struct WxLoginResponse {
    pub openid: String,
    pub session_key: String,
    pub unionid: String,
    pub errcode: i32,
    pub errmsg: String,
}

/// 微信小程序登录接口
pub async fn wx_login(jscode: Path<String>) -> Result<String, ServiceError> {
    let client = Client::default();
    let config = &CONFIG;
    let url =format!("https://api.weixin.qq.com/sns/jscode2session?appid={appid}&secret={secret}&js_code={code}&grant_type=authorization_code",appid=config.wechat_appid,secret=config.wechat_secret,code=jscode);
    let payload = client.get(url).send().await.unwrap().body().await;

    match payload {
        Ok(_) => Ok("logined".to_string()),
        _ => Err(ServiceError::BadRequest(
            "can't login to wechat".to_string(),
        )),
    }

    // match payload {
    //     Payload::Stream(res)
    // }
}
