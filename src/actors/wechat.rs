use crate::{config::CONFIG, errors::ServiceError};
use actix::{
    clock::{interval_at, Duration, Instant},
    prelude::*,
};
use actix_web::{client::Client, web::Data};
use std::sync::Mutex;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WxPong {
    pub access_token: Option<String>,
    pub expires_in: Option<u64>,

    pub errcode: Option<String>,
    pub errmsg: Option<String>,
}

pub struct Broadcaster {
    pub wx_client: Option<String>,
}

impl Broadcaster {
    pub fn create() -> Data<Mutex<Self>> {
        let me = Data::new(Mutex::new(Broadcaster::new()));
        Broadcaster::spawn_ping_wx(me.clone());

        me
    }

    fn new() -> Self {
        Broadcaster { wx_client: None }
    }

    fn spawn_ping_wx(me: Data<Mutex<Self>>) {
        actix_rt::spawn(async move {
            let mut task = interval_at(Instant::now(), Duration::from_secs(7));
            // let mut task = interval_at(Instant::now(), Duration::from_secs(7000));
            loop {
                me.lock().unwrap().ping_wx().await;
                task.tick().await;
            }
        })
    }

    /// 更新微信小程序token
    async fn ping_wx(&mut self) -> Result<(), ServiceError> {
        let url =format!("https://api.weixin.qq.com/cgi-bin/token?grant_type=client_credential&appid={appid}&secret={secret}",appid=&CONFIG.wechat_appid,secret=&CONFIG.wechat_secret);
        let body = Client::default()
            .clone()
            .get(url)
            .send()
            .await
            .map_err(|err| ServiceError::BadRequest(err.to_string()))?
            .body()
            .await
            .map_err(|err| ServiceError::BadRequest(err.to_string()))?;

        let res: WxPong = serde_json::from_slice(&body).unwrap();
        self.wx_client = res.access_token;
        Ok(())
    }
}
