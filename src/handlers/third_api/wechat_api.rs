// use actix_web::web::Data;

// fn spawn_ping_wx(me: Data<Mutex<Self>>) {
//     let mut task = interval(Duration::from_secs(7140));

//     actix_rt::spawn(async move {
//         while let Some(_) = task.next().await {
//             let res: Result<WxPong, ServiceError> =
//                 me.lock().expect("lock poll state faild").ping_wx().await;

//             if let Ok(wt) = res {
//                 if let Some(tk) = wt.access_token {
//                     me.lock()
//                         .expect("lock poll state faild")
//                         .state
//                         .insert(WECHAT_T, tk);
//                 }
//             }
//         }
//     })
// }

// /// 更新微信小程序token
// async fn ping_wx(&mut self) -> Result<WxPong, ServiceError> {
//     let url =format!("https://api.weixin.qq.com/cgi-bin/token?grant_type=client_credential&appid={appid}&secret={secret}",appid=&CONFIG.wechat_appid,secret=&CONFIG.wechat_secret);
//     let body = Client::default()
//         .clone()
//         .get(url)
//         .send()
//         .await
//         .map_err(|err| ServiceError::BadRequest(err.to_string()))?
//         .body()
//         .await
//         .map_err(|err| ServiceError::BadRequest(err.to_string()))?;

//     serde_json::from_slice::<WxPong>(&body)
//         .map_err(|e| ServiceError::InternalServerError(e.to_string()))
// }
