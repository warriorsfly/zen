use crate::{config::CONFIG, errors::ServiceError};
use actix::prelude::*;
use actix_web::{
    client::Client,
    web::{Data, ServiceConfig},
};
use std::time::Duration;

#[derive(Message)]
#[rtype(result = "()")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccessToken {
    pub access_token: Option<String>,
    pub expires_in: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WxTokenResponse {
    pub access_token: Option<String>,
    pub expires_in: Option<u64>,

    pub errcode: Option<String>,
    pub errmsg: Option<String>,
}

// Actor definition
pub struct Wechat {
    access_token: Option<String>,
    client: Data<Client>,
}

impl Actor for Wechat {
    type Context = Context<Self>;
}

// impl Default for Wechat {
//     fn default() -> Self {
//         Wechat { access_token: None }
//     }
// }

impl Hand ler<AccessToken> for Wechat {
    type Result = ();
    fn handle(&mut self, msg: AccessToken, ctx: &mut Self::Context)  {
        let res = request_wx_token(self.client);
        match res {
            Ok(access){
                if let Some(token)=access. {}
            }
        }
        // match res.access_token {
        //     None => {
        //         ctx.run_later(Duration::new(300, 0), move |act, _| {});
        //     }
        //     Some(access_token) => {
        //         self.access_token = access_token;
        //         ctx.run_later(Duration::new(msg.expires_in - 300, 0), move |act, _| {
        //             act.addr.do_send(msg.clone());
        //         });
        //     }
        // }
    }
}

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
pub async fn request_wx_token(client: Data<Client>) -> Result<AccessToken, ServiceError> {
    let url =format!("https://api.weixin.qq.com/cgi-bin/token?grant_type=client_credential&appid={appid}&secret={secret}",appid=&CONFIG.wechat_appid,secret=&CONFIG.wechat_secret);
    let body = client
        .get(url)
        .send()
        .await
        .map_err(|err| ServiceError::BadRequest(err.to_string()))?
        .body()
        .await
        .map_err(|err| ServiceError::BadRequest(err.to_string()))?;

    let res: WxTokenResponse =
        serde_json::from_slice(&body).map_err(|err| ServiceError::BadRequest(err.to_string()))?;

    if let Some(token) = res.access_token {
        let expires_in = res.expires_in.unwrap();
        Ok(AccessToken {
            access_token: Some(token),
            expires_in: expires_in,
        })
    } else {
        Err(ServiceError::InternalServerError(
            "wechat connection error".into(),
        ))
    }
}
