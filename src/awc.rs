use crate::{
    config::CONFIG,
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
    if !&CONFIG.wechat_appid.is_empty() && !&CONFIG.wechat_secret.is_empty() {
        // Start a new supervisor with redis actor
        add_wechat(client.clone(), state.clone());
    }
}

/// Add wechat access token to AppState

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WxPong {
    pub access_token: Option<String>,
    pub expires_in: Option<u64>,

    pub errcode: Option<u32>,
    pub errmsg: Option<String>,
}

fn add_wechat(client: Client, state: AppState<'static, String>) {
    actix_rt::spawn(async move {
        ping_wx(client.clone(), state.clone()).await;
        loop {
            let wt = get(state.clone(), WECHAT_T);
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

            ping_wx(client.clone(), state.clone()).await;
        }
    })
}

/// get wechat access token
async fn ping_wx(client: Client, state: AppState<'static, String>) {
    let url =format!("https://api.weixin.qq.com/cgi-bin/token?grant_type=client_credential&appid={appid}&secret={secret}",appid=&CONFIG.wechat_appid,secret=&CONFIG.wechat_secret);
    let body = client
        .get(url)
        .send()
        .await
        .expect("Bad request")
        .body()
        .await
        .expect("Bad response");

    let res: WxPong = serde_json::from_slice(&body).unwrap();
    if let Some(t) = res.access_token {
        set(state.clone(), WECHAT_T, t);
    } else {
        delete(state.clone(), WECHAT_T);
    }
}
