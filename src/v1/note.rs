use actix_web::{put, web, HttpRequest, HttpResponse, Responder};
use serenity::http::Http;

use crate::{auth::*, config, types::*};

#[put("/note")]
async fn add_note(
    note: web::Json<Note>,
    bot_http: web::Data<Http>,
    req: HttpRequest,
) -> impl Responder {
    if !valid_auth_token(req) {
        return HttpResponse::Unauthorized();
    }
    match config::CHANNEL.say(bot_http.get_ref(), &note.message).await {
        Ok(_) => HttpResponse::Ok(),
        Err(err) => {
            eprintln!("Failed to send message: {:?}", err);
            HttpResponse::InternalServerError()
        }
    }
}

// #[cfg(test)]
// mod test {
//     use std::sync::Arc;

//     use actix_web::{test::TestRequest, web};
//     use serenity::http::Http;

//     use crate::types::Note;

//     use super::add_note;

//     #[actix_web::test]
//     async fn test_unauthorized() {
//         let note = web::Json(Note {
//             message: "Hello world".into(),
//         });
//         let bot_http = web::Data::from(Arc::new(Http::default()));
//         let req = TestRequest::default().to_http_request();
//         let resp = add_note(note, bot_http, req).await;
//     }
// }
