use crate::{config::CONFIG, errors::ServiceError};
use actix::prelude::*;
use actix_web::{
    client::Client,
    web::{Data, ServiceConfig},
};
use std::time::Duration;

#[derive(Message)]
#[rtype(result = "()")]
pub struct WxMessage(Option<String>, u64);

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WxAccessToken {
    pub access_token: Option<String>,
    pub expires_in: Option<u64>,

    pub errcode: Option<String>,
    pub errmsg: Option<String>,
}

// Actor definition
pub struct Wechat {
    access_token: Option<String>,
}

impl Actor for Wechat {
    type Context = Context<Self>;
}

// impl Default for Wechat {
//     fn default() -> Self {
//         Wechat { access_token: None }
//     }
// }

// impl Handler<WxMessage> for Wechat {
//     type Result = ();
//     fn handle(&mut self, msg: WxMessage, ctx: &mut Self::Context)->Self.Result {

//     }
// }

// pub fn add_wechat(cfg: &mut ServiceConfig) {
//     if !!&CONFIG.wechat_appid.is_empty() && !&CONFIG.wechat_secret.is_empty() {
//         return;
//     }
//     // Start a new supervisor with redis actor

//     let wechat = Wechat {
//         access_token: None,
//         addr: add,
//     };
//     cfg.data(wechat.addr);
// }

/// 更新微信小程序token
pub async fn request_wx_token(client: Data<Client>) -> Result<WxAccessToken, ServiceError> {
    let url =format!("https://api.weixin.qq.com/cgi-bin/token?grant_type=client_credential&appid={appid}&secret={secret}",appid=&CONFIG.wechat_appid,secret=&CONFIG.wechat_secret);
    let body = client
        .get(url)
        .send()
        .await
        .map_err(|err| ServiceError::BadRequest(err.to_string()))?
        .body()
        .await
        .map_err(|err| ServiceError::BadRequest(err.to_string()))?;

    let res: WxAccessToken =
        serde_json::from_slice(&body).map_err(|err| ServiceError::BadRequest(err.to_string()))?;

    Ok(res)
}
