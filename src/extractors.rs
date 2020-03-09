use crate::auth::decode_jwt;
use crate::models::account::auth::AuthAccount;
use actix_web::{
    dev::Payload,
    web::{HttpRequest, HttpResponse},
    Error, FromRequest,
};

use actix_identity::RequestIdentity;
use futures::future::{err, ok, Ready};

/// Extractor for pulling the identity out of a request.
///
/// Simply add "user: AuthAccount" to a handler to invoke this.
impl FromRequest for AuthAccount {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let identity = RequestIdentity::get_identity(req);

        if let Some(identity) = identity {
            let private_claim = decode_jwt(&identity).unwrap();
            return ok(AuthAccount {
                id: private_claim.user_id.to_string(),
                phone: private_claim.phone,
            });
        }
        err(HttpResponse::Unauthorized().into())
    }
}
