use crate::config::CONFIG;
use actix_web::{client::Client, web::Path};

pub struct WxLoginResponse {
    pub openid: String,
    pub session_key: String,
    pub unionid: String,
    pub errcode: i32,
    pub errmsg: String,
}

// pub async fn wx_login(jscode: Path<String>) {
//     let client = Client::default();
//     let config = &CONFIG;
//     let url =format!("https://api.weixin.qq.com/sns/jscode2session?appid={appid}&secret={secret}&js_code={code}&grant_type=authorization_code",appid=config.wechat_appid,secret=config.wechat_secret,code=jscode);
//     let payload = client.get(url).send().await.unwrap().body().await.unwrap();

//     match payload {
//         Payload(res) if res
//     }
// }
