use salvo::jwt_auth::{ConstDecoder, CookieFinder, HeaderFinder, QueryFinder};
use salvo::prelude::*;
use util::JwtClaims;


pub fn auth_hoop() -> JwtAuth<JwtClaims, ConstDecoder> {
    let config = config::get();
    JwtAuth::new(ConstDecoder::from_secret(config.jwt.secret.as_bytes()))
        .finders(vec![
            Box::new(HeaderFinder::new()),
            Box::new(QueryFinder::new("token")),
            Box::new(CookieFinder::new("jwt_token")),
        ])
        .force_passed(false)
}
