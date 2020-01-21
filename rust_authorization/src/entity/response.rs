#[derive(Debug,Rerialize,Deserialize)]
pub struct Response<T> {
    pub message: String,
    pub data:T,
}

impl<T> Response<T>{
    pub fn new(message:&str,data:T)->Response<T>{
        Response{
            message:message.to_string(),
            data,
        }
    }
}
