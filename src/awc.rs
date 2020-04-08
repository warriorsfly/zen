use crate::{
    config::CONFIG,
    errors::ServiceError,
    state::{delete, get, new_state, set, AppState, WECHAT_T},
};
use actix::clock::{interval_at, Duration, Instant};
use actix_web::{client::Client, web::ServiceConfig};

/// Add awc to actix data if the URL is set
pub fn add_awc(cfg: &mut ServiceConfig) {
    let client = Client::default();
    cfg.data(client.clone());
    let state = new_state::<String>();
    cfg.data(state.clone());
}

/// Add wechat access token to AppState
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WxPong {
    pub access_token: Option<String>,
    pub expires_in: Option<u64>,

    pub errcode: Option<u32>,
    pub errmsg: Option<String>,
}

pub fn add_state() -> AppState<'static, String> {
    let state = new_state::<String>();
    let data = state.clone();

    if !&CONFIG.wechat_appid.is_empty() && !&CONFIG.wechat_secret.is_empty() {
        // Start a new supervisor with redis actor
        actix_rt::spawn(async move {
            let res = ping_wx().await;
            if let Ok(res) = res {
                if let Some(t) = res.access_token {
                    set(data.clone(), WECHAT_T, t);
                } else {
                    delete(data.clone(), WECHAT_T);
                }
            }
            loop {
                let wt = get(data.clone(), WECHAT_T);
                match wt {
                    Some(_) => {
                        let mut task = interval_at(Instant::now(), Duration::from_secs(7140));
                        task.tick().await;
                    }
                    None => {
                        let mut task = interval_at(Instant::now(), Duration::from_secs(30));
                        task.tick().await;
                    }
                }

                let res = ping_wx().await;
                if let Ok(res) = res {
                    if let Some(t) = res.access_token {
                        set(data.clone(), WECHAT_T, t);
                    } else {
                        delete(data.clone(), WECHAT_T);
                    }
                }
            }
        });
    }
    state
}

/// get wechat access token
async fn ping_wx() -> Result<WxPong, ServiceError> {
    let url =format!("https://api.weixin.qq.com/cgi-bin/token?grant_type=client_credential&appid={appid}&secret={secret}",appid=&CONFIG.wechat_appid,secret=&CONFIG.wechat_secret);
    let body = Client::default()
        .get(url)
        .send()
        .await
        .map_err(|e| ServiceError::BadRequest(e.to_string()))?
        .body()
        .await
        .map_err(|e| ServiceError::BadRequest(e.to_string()))?;

    serde_json::from_slice::<WxPong>(&body)
        .map_err(|e| ServiceError::InternalServerError(e.to_string()))
}
