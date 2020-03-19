use crate::auth::decode_jwt;
use crate::{constants, errors::ServiceError, models::account::auth::AuthClaim};
use actix_web::{dev::Payload, web::HttpRequest, FromRequest};

// use actix_identity::RequestIdentity;
use futures::future::{err, ok, Ready};

/// Extractor for pulling the user out of a request.
///
/// Simply add "user: AuthClaim" to a handler to invoke this.
impl FromRequest for AuthClaim {
    type Error = ServiceError;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        if let Some(authen_header) = req.headers().get(constants::AUTHORIZATION) {
            // info!("Parsing authorization header...");
            if let Ok(authen_str) = authen_header.to_str() {
                if authen_str.to_lowercase().starts_with("bearer") {
                    // info!("Parsing token...");
                    let jwt = authen_str[6..authen_str.len()].trim();
                    let claim = decode_jwt(jwt).unwrap();
                    return ok(AuthClaim {
                        uid: claim.uid.to_string(),
                        identifier: claim.identifier,
                        identity_type: claim.identity_type,
                    });
                }
            }
        }

        err(ServiceError::Unauthorized("Unauthorized".into()))
    }
}
