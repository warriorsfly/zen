use crate::jwt::{decode_jwt, Claims};
use crate::{constants, errors::ServError};
use actix_web::{dev::Payload, web::HttpRequest, FromRequest};
use futures::future::{err, ok, Ready};

/// Extractor for pulling the user out of a request.
///
/// Simply add "user: AuthClaim" to a handler to invoke this.
impl FromRequest for Claims {
    type Error = ServError;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        if let Some(authen_header) = req.headers().get(constants::AUTHORIZATION) {
            // info!("Parsing authorization header...");
            if let Ok(authen_str) = authen_header.to_str() {
                if authen_str.starts_with("bearer")
                    || authen_str.starts_with("Bearer")
                    || authen_str.starts_with("BEARER")
                {
                    // info!("Parsing token...");
                    let jwt = authen_str[6..authen_str.len()].trim();
                    let claim = decode_jwt(jwt).unwrap();
                    return ok(claim);
                }
            }
        }

        err(ServError::Unauthorized("Unauthorized".into()))
    }
}
