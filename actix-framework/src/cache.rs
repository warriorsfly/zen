use crate::errors::ServiceError;
use actix::prelude::*;
use actix_redis::{Command, RedisActor};
use actix_web::web::{Data, ServiceConfig};
use redis_async::resp::{FromResp, RespValue};

pub type Redis = Data<Addr<RedisActor>>;

/// Retrieve an entry in redis
#[allow(dead_code)]
pub async fn get<'a>(redis: Redis, key: &'a str) -> Result<String, ServiceError> {
    let command = resp_array!["GET", key];
    send(redis, command).await
}

/// Insert or update an entry in redis
#[allow(dead_code)]
pub async fn set<'a>(redis: Redis, key: &'a str, value: &'a str) -> Result<String, ServiceError> {
    let command = resp_array!["SET", key, value];
    send(redis, command).await
}

/// Delete an entry in redis
#[allow(dead_code)]
pub async fn delete<'a>(redis: Redis, key: &'a str) -> Result<String, ServiceError> {
    let command = resp_array!["DEL", key];
    send(redis, command).await
}

/// Send a command to the redis actor
async fn send<'a>(redis: Redis, command: RespValue) -> Result<String, ServiceError> {
    let error_message = format!("Could not send {:?} command to Redis", command);
    let error = ServiceError::InternalServerError(error_message.into());
    let response = redis.send(Command(command)).await.map_err(|_| error)?;
    match response {
        Ok(message) => Ok::<String, _>(FromResp::from_resp(message).unwrap_or("".into())),
        Err(message) => Err(ServiceError::InternalServerError(format!("{:?}", message))),
    }
}

/// Add the redis actor to actix data if the URL is set
#[allow(dead_code)]
pub fn add_cache(cfg: &mut ServiceConfig, url: &str) {
    if !url.is_empty() {
        // Start a new supervisor with redis actor
        let cache = RedisActor::start(url);
        cfg.data(cache);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const REDIS_URL: &str = "127.0.0.1:6379";
    fn get_cache() -> Redis {
        let cache = RedisActor::start(REDIS_URL);
        Data::new(cache)
    }

    #[actix_rt::test]
    async fn it_creates_new_application_cache_and_sets_and_reads_it() {
        if !REDIS_URL.is_empty() {
            let cache = get_cache();
            set(cache.clone(), "testing", "123").await.unwrap();
            let value = get(cache, "testing").await.unwrap();
            assert_eq!(value, "123");
        } else {
            assert!(true);
        }
    }

    #[actix_rt::test]
    async fn it_removes_an_entry_in_application_cache() {
        if !REDIS_URL.is_empty() {
            let cache = get_cache();
            set(cache.clone(), "testing", "123").await.unwrap();
            let value = get(cache.clone(), "testing").await.unwrap();
            assert_eq!(value, "123");
            delete(cache.clone(), "testing").await.unwrap();
            let value = get(cache, "testing").await.unwrap();
            assert_eq!(value, "");
        } else {
            assert!(true);
        }
    }
}
