mod note;

use crate::auth::valid_auth_token;
use actix_web::{dev::Service, web, HttpResponse, Scope};
use serenity::FutureExt;

pub fn get_service() -> Scope<dyn ScopeEndpoint> {
    web::scope("/v1")
        .wrap_fn(|req, srv| {
            srv.call(req).map(|res| {
                if !valid_auth_token(&req.headers()) {
                    Ok(req.into_response(HttpResponse::Unauthorized().finish()))
                } else {
                    res
                }
            })
        })
        .service(note::add_note)
}
