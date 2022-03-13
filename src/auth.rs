use crate::config::*;
use actix_web::{http, HttpRequest};

pub fn valid_auth_token(req: HttpRequest) -> bool {
    req.headers()
        .get(http::header::AUTHORIZATION)
        .filter(|&value| value == AUTH)
        .is_some()
}
